use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_yaml::{Error, Value};
use void::Void;

use crate::custom_types::OneOrMany;

/// You can schedule a workflow to run at specific UTC times using POSIX cron
/// syntax. Scheduled workflows run on the latest commit on the default or base
/// branch. The shortest interval you can run scheduled workflows is once every 5
/// minutes.
type Schedule = Vec<CronSchedule>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CronSchedule {
    // TODO: validate cron string
    cron: String,
}

// TODO: enumerate these, starting with the common ones
/// Event types
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
enum Event {
    Push(Value),
    PullRequest(Value),
    WorkflowDispatch(Value),
    RepositoryDispatch(Value),
    CheckRun(Value),
    CheckSuite(Value),
    Create(Value),
    Delete(Value),
    Deployment(Value),
    DeploymentStatus(Value),
    Fork(Value),
    Gollum(Value),
    IssueComment(Value),
    Issues(Value),
    Label(Value),
    Milestone(Value),
    PageBuild(Value),
    Project(Value),
    ProjectCard(Value),
    ProjectColumn(Value),
    Public(Value),
    PullRequestReview(Value),
    PullRequestReviewComment(Value),
    PullRequestTarget(Value),
    RegistryPackage(Value),
    Release(Value),
    Status(Value),
    Watch(Value),
    WorkflowRun(Value),
}

/// Trigger types for a workflow.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
enum Trigger {
    Events(OneOrMany<Event>),
    Schedule(Schedule),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DefaultSettings {
    shell: Option<String>,
    working_directory: Option<String>,
}

/// Provide default shell and working-directory to all run steps in the job.
/// Context and expression are not allowed in this section.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Defaults {
    run: DefaultSettings,
}

/// The environment that the job references. All environment protection rules
/// must pass before a job referencing the environment is sent to a runner.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Environment {
    name: String,
    url: Option<String>,
}

// TODO
type Matrix = Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Strategy {
    matrix: Option<Matrix>,
    fail_fast: Option<bool>,
    max_parallel: Option<i32>,
}

/// Runs command-line programs using the operating system's shell. If you do not
/// provide a name, the step name will default to the text specified in the run
/// command.
type ShellCommand = String;

/// Steps can run commands, run setup tasks, or run an action in your
/// repository, a public repository, or an action published in a Docker registry.
/// Not all steps run actions, but all actions run as a step. Each step runs in its
/// own process in the runner environment and has access to the workspace and
/// filesystem.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Step {
    /// A name for your step to display on GitHub.
    name: Option<String>,

    /// A unique identifier for the step. You can use the id to reference the
    /// step in contexts.
    id: Option<String>,

    /// You can use the if conditional to prevent a step from running unless a
    /// condition is met. You can use any supported context and expression to
    /// create a conditional.
    #[serde(rename = "if")]
    run_if: Option<String>,

    /// Selects an action to run as part of a step in your job. An action is a reusable
    /// unit of code. You can use an action defined in the same repository as the
    /// workflow, a public repository, or in a published Docker container image.
    uses: String,

    run: Option<ShellCommand>,

    /// A map of the input parameters defined by the action. Each input parameter is a
    /// key/value pair. Input parameters are set as environment variables. The variable
    /// is prefixed with INPUT_ and converted to upper case.
    ///
    /// Both `entrypoint` and `args` are supported and override a docker image's default
    /// values for those variables.
    #[serde(default)]
    with: HashMap<String, String>,

    /// Sets environment variables for steps to use in the runner environment. You can
    /// also set environment variables for the entire workflow or a job.
    #[serde(default)]
    env: Env,

    /// Prevents a job from failing when a step fails. Set to true to allow a job to
    /// pass when this step fails.
    continue_on_error: Option<bool>,

    /// The maximum number of minutes to run the step before killing the process.
    timeout_minutes: Option<i32>,
}

/// A container to run any steps in a job that don't already specify a container.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Container {
    name: String,
    credentials: Option<HashMap<String, String>>,
    env: Option<Env>,
    #[serde(default)]
    ports: Vec<i32>,
    #[serde(default)]
    volumes: Vec<i32>,
    #[serde(default)]
    options: Vec<String>,
}
impl FromStr for Container {
    // This implementation of `from_str` can never fail, so use the impossible
    // `Void` type as the error type.
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Container {
            name: s.to_string(),
            ..Default::default()
        })
    }
}

// TODO
/// Used to host service containers for a job in a workflow. Service containers
/// are useful for creating databases or cache services like Redis.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Service {
    name: String,
    #[serde(default)]
    ports: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Job {
    /// The name of the job displayed on GitHub.
    name: Option<String>,

    /// Identifies any jobs that must complete successfully before this job
    /// will run. It can be a string or array of strings. If a job fails, all jobs that
    /// need it are skipped unless the jobs use a conditional expression that causes
    /// the job to continue.
    #[serde(default)]
    needs: Vec<String>,

    /// The type of machine to run the job on. The machine can be either a GitHub-hosted
    /// runner or a self-hosted runner.
    runs_on: OneOrMany<String>,

    /// The environment that the job references. All environment protection rules must
    /// pass before a job referencing the environment is sent to a runner.
    environment: Option<Environment>,

    /// A map of outputs for a job. Job outputs are available to all downstream jobs
    /// that depend on this job.
    outputs: Option<HashMap<String, Output>>,

    /// A map of environment variables that are available to all steps in the job. You
    /// can also set environment variables for the entire workflow or an individual step.
    #[serde(default)]
    env: Env,

    /// A map of default settings that will apply to all steps in the job. You can also
    /// set default settings for the entire workflow.
    defaults: Option<Defaults>,

    /// You can use the if conditional to prevent a job from running unless a condition
    /// is met. You can use any supported context and expression to create a conditional.
    #[serde(rename = "if")]
    run_if: Option<String>,

    /// A job contains a sequence of tasks called steps. Because steps run in
    /// their own process, changes to environment variables are not preserved
    /// between steps. GitHub provides built-in steps to set up and complete a job.
    #[serde(default)]
    steps: Vec<Step>,

    /// The maximum number of minutes to run the step before killing the process.
    timeout_minutes: Option<i32>,

    /// A strategy creates a build matrix for your jobs. You can define different
    /// variations to run each job in.
    strategy: Option<Strategy>,

    /// Prevents a job from failing when a step fails. Set to true to allow a job to
    /// pass when this step fails.
    continue_on_error: Option<String>,

    /// If you have steps that use both script and container actions, the container
    /// actions will run as sibling containers on the same network with the same volume mounts.
    container: Option<Container>,

    /// The runner automatically creates a Docker network and manages the life
    /// cycle of the service containers.
    #[serde(default)]
    services: Vec<Service>,
}

type Env = HashMap<String, String>;

type JobMap = HashMap<String, Job>;

// TODO: determine if outputs _need_ to be an expression and validate
type Output = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Workflow {
    /// The name of your workflow. GitHub displays the names of your workflows on your
    /// repository's actions page. If you omit name, GitHub sets it to the workflow
    /// file path relative to the root of the repository.
    name: Option<String>,

    /// The name of the GitHub event that triggers the workflow. You can provide a
    /// single event string, array of events, array of event types, or an event
    /// configuration map that schedules a workflow or restricts the execution of a
    /// workflow to specific files, tags, or branch changes.
    on: Trigger,

    /// A map of environment variables that are available to all jobs and steps
    /// in the workflow. You can also set environment variables that are only
    /// available to a job or step.
    env: Option<Env>,

    /// A map of default settings that will apply to all jobs in the workflow. You can
    /// also set default settings that are only available to a job.
    defaults: Option<Defaults>,

    /// A workflow run is made up of one or more jobs. Jobs run in parallel by
    /// default. To run jobs sequentially, you can define dependencies on other jobs
    /// using the jobs.<job_id>.needs keyword.
    jobs: JobMap,
}

impl Workflow {
    pub fn parse_str(input: &str) -> Result<Self, Error> {
        serde_yaml::from_str(&input)
    }
}
