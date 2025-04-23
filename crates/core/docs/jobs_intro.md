In the Blueprint SDK a "job" is an async function that accepts zero or more
["extractors"](crate::extract) as arguments and returns something that
can be converted [into a job result](crate::job::result).

Jobs are where your application logic lives, and Blueprints are built by routing between one or many of them.

[`debug_job`]: https://docs.rs/blueprint-macros/latest/blueprint_macros/attr.debug_job.html
