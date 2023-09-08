use egui::{Color32, Context, Frame, Id, Ui};
use std::borrow::Cow;

pub trait ItemTrait {
    const TITLE: &'static str;
    const INNER_MARGIN: f32 = 2.0;
    const OUTER_MARGIN: f32 = 2.0;
    const ROUNDING: f32 = 5.0;
    const STRIPED: bool = true;

    type Data<'a>: Copy;

    /// 在该列表中的唯一标识
    fn id(&self, _data: Self::Data<'_>) -> Id;

    /// 正常状态下元素的frame配置
    fn style_normal(&self, frame: &mut Frame) {}

    /// 元素clicked时的frame配置
    fn style_clicked(&self, frame: &mut Frame) {
        frame.fill = Color32::LIGHT_GREEN;
    }

    /// 元素hovered时的frame配置
    fn style_hovered(&self, frame: &mut Frame) {
        frame.fill = Color32::LIGHT_YELLOW;
    }

    /// 在列表中的显示
    fn show(
        &self,
        selected: bool,
        hovered: bool,
        ctx: &Context,
        ui: &mut Ui,
        _data: Self::Data<'_>,
    );

    /// hover时展示的文字
    fn hovered_text(&self) -> Option<Cow<'_, str>> {
        None
    }

    /// 在绘制完所有元素后调用，传递当前选择的元素
    fn selected_item(&self, _data: Self::Data<'_>) {}

    /// 是否符合搜索条件
    fn on_search(&self, text: &str, _data: Self::Data<'_>) -> bool;
}
