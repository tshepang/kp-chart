use std::fmt::{self, Display, Formatter};
use std::iter::Cycle;
use std::slice::Iter;

use data::{Ability, Job, Person};

#[derive(Clone, Debug)]
pub struct Day {
    name: String,
    jobs: Vec<(Job, Vec<Person>)>,
}

impl Day {
    pub fn new(
        name: String,
        jobs: Vec<Job>,
        children: &mut Cycle<Iter<Person>>,
        teens: &mut Cycle<Iter<Person>>,
        adults: &mut Cycle<Iter<Person>>,
    ) -> Self {
        let mut day_jobs = jobs
            .clone()
            .into_iter()
            .map(|j| (j, Vec::<Person>::new()))
            .collect::<Vec<_>>();

        // pass through all children jobs first
        for (job, ref mut workers) in day_jobs.iter_mut() {
            for ability in job.people().iter() {
                match *ability {
                    Ability::Child => {
                        workers.push(children.next().expect("ran out of children").clone())
                    }
                    Ability::Teen => workers.push(teens.next().expect("ran out of teens").clone()),
                    Ability::Adult => {
                        workers.push(adults.next().expect("ran out of adults").clone())
                    }
                }
            }
        }

        Self {
            name,
            jobs: day_jobs,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn jobs(&self) -> &[(Job, Vec<Person>)] {
        &self.jobs
    }

    pub fn get_job_people(&self, job: usize) -> &[Person] {
        &self.jobs[job].1
    }
}

impl Display for Day {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        for (job, people) in self.jobs.iter() {
            write!(fmt, "{}: ", job)?;
            for person in people.iter() {
                write!(fmt, "{}, ", person)?;
            }
            writeln!(fmt, "")?;
        }
        Ok(())
    }
}
