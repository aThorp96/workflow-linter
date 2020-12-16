use std::collections::HashMap;

enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> OneOrMany<T> {
    fn into_vec(self) -> Vec<T> {
        match self {
            OneOrMany::One(i) => vec![i],
            OneOrMany::Many(l) => l,
        }
    }
}

/// An event trigger for a workflow.
enum Event {
    // TODO: enumerate all events
    Push,
    PullRequest,
}

/// You can schedule a workflow to run at specific UTC times using POSIX cron
/// syntax. Scheduled workflows run on the latest commit on the default or base
/// branch. The shortest interval you can run scheduled workflows is once every 5
/// minutes.
type Schedule = Vec<CronSchedule>;
struct CronSchedule {
    // TODO: validate cron string
    cron: String,
}

/// Trigger types for a workflow.
enum Trigger {
    Events(OneOrMany<Event>),
    Schedule(Schedule),
}

struct DefaultSettings {
    shell: Option<String>,
    working_directory: Option<String>,
}

/// Provide default shell and working-directory to all run steps in the job.
/// Context and expression are not allowed in this section.
struct Defaults {
    run: DefaultSettings,
}

/// The environment that the job references. All environment protection rules
/// must pass before a job referencing the environment is sent to a runner.
struct Environment {
    name: String,
    url: Option<String>,
}

// TODO
struct Matrix {}

// TODO
struct Strategy {
    matrix: Option<Matrix>,
    fail_fast: Option<bool>,
    max_parallel: Option<i32>,
}

// TODO
/// Steps can run commands, run setup tasks, or run an action in your
/// repository, a public repository, or an action published in a Docker registry.
/// Not all steps run actions, but all actions run as a step. Each step runs in its
/// own process in the runner environment and has access to the workspace and
/// filesystem.
struct Step {}

// TODO
/// A container to run any steps in a job that don't already specify a container.
struct Container {}

// TODO
/// Used to host service containers for a job in a workflow. Service containers
/// are useful for creating databases or cache services like Redis.
struct Service {}

// TODO
struct Job {
    /// The name of the job displayed on GitHub.
    name: String,

    /// Identifies any jobs that must complete successfully before this job
    /// will run. It can be a string or array of strings. If a job fails, all jobs that
    /// need it are skipped unless the jobs use a conditional expression that causes
    /// the job to continue.
    needs: Option<Vec<String>>,

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
    env: Option<Env>,

    /// A map of default settings that will apply to all steps in the job. You can also
    /// set default settings for the entire workflow.
    defaults: Option<Defaults>,

    /// You can use the if conditional to prevent a job from running unless a condition
    /// is met. You can use any supported context and expression to create a conditional.
    run_if: Option<String>,

    /// A job contains a sequence of tasks called steps. Because steps run in
    /// their own process, changes to environment variables are not preserved
    /// between steps. GitHub provides built-in steps to set up and complete a job.
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

    /// The runner automatically creates a Docker network and manages the life cycle of the service containers.
    services: Option<Vec<Service>>,
}

type Env = HashMap<String, String>;
type JobMap = HashMap<String, Job>;

// TODO: determine if outputs _need_ to be an expression and validate
type Output = String;

struct Workflow {
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

fn main() {
    println!("Imagine having two write this in marshmallow.py");
}
