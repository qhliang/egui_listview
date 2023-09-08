use crate::detail::ViewGroupDetail;
use crate::group_new::ViewGroupNew;

pub(crate) enum State {
    GroupNew(ViewGroupNew),
    GroupDetail(ViewGroupDetail),
}

impl State {
    pub fn as_app(&mut self) -> &mut dyn eframe::App {
        match self {
            State::GroupNew(a) => a,
            State::GroupDetail(a) => a,
        }
    }
}
