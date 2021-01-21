use leo_asg::Program;

pub trait ASGStage {
    fn apply(asg: &mut Program);
}
