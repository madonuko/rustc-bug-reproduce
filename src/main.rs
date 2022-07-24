use patternfly_yew::{
    ColumnIndex, SharedTableModel, Table, TableHeader, TableMode, TableRenderer,
};
use yew::prelude::*;


#[derive(PartialEq, Debug, Clone)]
struct ArtifactDisplay<'a> {
    timestamp: &'a String,
}


impl TableRenderer for ArtifactDisplay<'_> {
    fn render(&self, _: ColumnIndex) -> Html {
        html! {}
    }
}

fn artifacts() -> Html {
    let header = html_nested! { <TableHeader/> };
    let model: SharedTableModel<_> = vec![].into();
    html! {
        <Table<SharedTableModel<ArtifactDisplay>>
            mode={TableMode::CompactExpandable}
            header={header}
            entries={model}
        />
    }
}
fn main () {}
