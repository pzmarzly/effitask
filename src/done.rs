use relm_attributes::widget;
use widgets::Tasks;
use widgets::tasks::Msg::{Complete, Edit};

#[derive(Msg)]
pub enum Msg {
    Complete(Box<::tasks::Task>),
    Edit(Box<::tasks::Task>),
    Update,
}

impl Widget {
    fn update_tasks(&mut self) {
        let list = ::application::tasks();
        let tasks = list.tasks.iter().filter(|x| x.finished).cloned().collect();

        self.tasks.emit(::widgets::tasks::Msg::Update(tasks));
    }
}

#[widget]
impl ::relm::Widget for Widget {
    fn model(_: ()) -> () {}

    fn update(&mut self, event: Msg) {
        use self::Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Update => self.update_tasks(),
        }
    }

    view!
    {
        #[name="tasks"]
        Tasks {
            Complete(ref task) => Msg::Complete(task.clone()),
            Edit(ref task) => Msg::Edit(task.clone()),
        }
    }
}
