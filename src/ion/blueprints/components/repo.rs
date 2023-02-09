use crate::{blueprints::*, utils::git};
use log::debug;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Info {
    pub url: String,
    pub remote: String,
    pub branch: String,
    pub ignore: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitRepo {
    branch: Option<String>,
    #[serde(default)]
    ssh: bool, // default is false
    #[serde(default = "GitRepo::default_suffix")]
    suffix: String,
    #[serde(default = "GitRepo::default_ignore")]
    ignore: String,
}

impl GitRepo {
    pub fn default_suffix() -> String {
        ".jl".to_string()
    }

    pub fn default_ignore() -> String {
        "./.gitignore.hbs".into()
    }
}

impl Blueprint for GitRepo {
    fn collect(&self, _t: &Template, config: &Config, ctx: &mut Context) -> RenderResult {
        let branch = match &self.branch {
            Some(b) => b.to_owned(),
            None => "main".to_string(),
        };

        let user = &*ctx.project.git.as_ref().unwrap().user;
        let package = &*ctx.project.name;
        let repo = package.to_string() + &self.suffix;

        let remote = if self.ssh {
            format!(r#"git@github.com:{user}/{repo}.git"#)
        } else {
            format!("https://github.com/{user}/{repo}.git")
        };
        let url = format!("https://github.com/{user}/{repo}");

        ctx.repo = Some(Info {
            url,
            remote,
            branch,
            ignore: vec![],
        });
        Ok(())
    }

    // 1. git init
    // 2. git remote add origin
    // if default branch (main, usually) is not <branch>
    // 3. git checkout -b <branch>
    // 4. git branch -D main
    // 5. git branch -m <branch>
    fn render(&self, _t: &Template, config: &Config, ctx: &Context) -> RenderResult {
        self.ignore.as_template()?.render(ctx, ".gitignore")?;
        let repo = ctx.repo.as_ref().unwrap();
        let remote = &repo.remote;
        let branch = &repo.branch;

        std::process::Command::new("git").arg("init").output()?;

        if let Some(current_branch) = git_current_branch() {
            if &current_branch != branch {
                git_checkout(branch)?;
                git_delete_branch(&current_branch)?;
                std::process::Command::new("git")
                    .arg("branch")
                    .arg("-m")
                    .arg(branch)
                    .status()?;
            }
        } else {
            git_checkout(branch)?;
        }

        debug!("git remote add origin {}", remote);
        std::process::Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(remote)
            .status()?;
        Ok(())
    }

    // 1. git add -A
    fn post_render(&self, _t: &Template, config: &Config, _ctx: &Context) -> RenderResult {
        debug!("git add -A");
        std::process::Command::new("git")
            .arg("add")
            .arg("-A")
            .status()?;
        debug!("git commit -m 'files generated by ion'");
        git::commit(&std::env::current_dir()?, "files generated by ion")?;
        Ok(())
    }
}
