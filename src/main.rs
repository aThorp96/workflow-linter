use std::collections::HashMap;

enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
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

// TODO
/// Provide default shell and working-directory to all run steps in the job.
/// Context and expression are not allowed in this section.
struct Defaults {}

// TODO
struct Job {
    /// The name of the job displayed on GitHub.
    name: String,

    /// Identifies any jobs that must complete successfully before this job
    /// will run. It can be a string or array of strings. If a job fails, all jobs that
    /// need it are skipped unless the jobs use a conditional expression that causes
    /// the job to continue.
    needs: Option<Vec<String>>,
}

type Env = HashMap<String, String>;
type JobMap = HashMap<String, Job>;

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
