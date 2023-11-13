use dioxus::prelude::*;
use dioxus_sortable::{use_sorter, NullHandling, PartialOrdBy, SortBy, Sortable, Th, ThStatus};

fn main() {
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    // Trigger pulling our data "externally"
    let future = use_future(cx, (), |_| load_prime_ministers());

    cx.render(rsx! {
        h1 { "Birthplaces of British prime ministers" }
        future.value().map_or_else(
            // Show a loading message while the data is being fetched
            || rsx!{
                p { "Loading..." }
            },
            // Pass the data onto our table component
            |data| rsx!{
                PrimeMinisters{ data: data.to_vec(), }
            })
    })
}

/// Creates a sortable table of prime ministers and their birthplaces. Can be filtered by name.
///
/// Each column header can be clicked to sort by that column. The current sort state is displayed in the header.
#[allow(non_snake_case)]
#[inline_props]
fn PrimeMinisters(cx: Scope, data: Vec<Purchase>) -> Element {
    // Sorter hook must be called unconditionally
    let sorter = use_sorter::<PurchaseField>(cx);
    let name = use_state(cx, || "".to_string());

    // Filter the data
    let mut data = data
        .to_owned()
        .into_iter()
        .filter(|row| row.description.to_lowercase().contains(&name.get().to_lowercase()))
        .collect::<Vec<_>>();
    // Sort the data. Unlike use_sorter, may be skipped
    sorter.sort(data.as_mut_slice());

    cx.render(rsx! {
        // Our simple search box
        input {
            placeholder: "Search by name",
            oninput: move |evt| name.set(evt.value.clone()),
        }

        // Render a table like we would any other except for the `Th` component
        table {
            thead {
                tr {
                    // The `Th` helper component is used to render a sortable column header
                    Th { sorter: sorter, field: PurchaseField::Id, "Id" }
                    // The `Th` helper component is used to render a sortable column header
                    Th { sorter: sorter, field: PurchaseField::Name, "Name" }
                    // It will display an arrow to indicate the current sort direction and state
                    Th { sorter: sorter, field: PurchaseField::Description, "Description" }
                    Th { sorter: sorter, field: PurchaseField::Price, "Price" }
                    Th { sorter: sorter, field: PurchaseField::Quantity, "Quantity" }
                }
            }
            tbody {
                // Iterate over our Purchase data like we would any other.
                data.iter().map(|row| {
                    rsx! {
                        tr {
                            td { "{row.id}" }
                            td { "{row.name}" }
                            td { "{row.description}" }
                            td { "{row.price}" }
                            td { "{row.quantity}" }
                        }
                    }
                })
            }
        }
    })
}

/// Our per-row data type that we want to sort
#[derive(Clone, Debug, PartialEq)]
struct Purchase {
    id: usize,
    name: String,
    description: String,
    price: f64,
    quantity: usize,
}

/// This is the field we want to sort by. Each variant corresponds to a column in our table or field in our Purchase struct. Keep it simple, use `{struct}Field`.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
enum PurchaseField {
    Id,
    #[default]
    Name,
    Description,
    Price,
    Quantity,
}

/// This trait decides how our rows are sorted
impl PartialOrdBy<Purchase> for PurchaseField {
    fn partial_cmp_by(&self, a: &Purchase, b: &Purchase) -> Option<std::cmp::Ordering> {
        match self {
            // Most values like Strings, integers and f64 require no special treatment and partial_cmp can be used directly.
            PurchaseField::Name => a.name.partial_cmp(&b.name),
            PurchaseField::Description => a.description.partial_cmp(&b.description),
            PurchaseField::Id => a.id.partial_cmp(&b.id),
            PurchaseField::Price => a.price.partial_cmp(&b.price),
            PurchaseField::Quantity => a.quantity.partial_cmp(&b.quantity),
        }
    }
}

/// This trait decides how fields (columns) may be sorted
impl Sortable for PurchaseField {
    fn sort_by(&self) -> Option<SortBy> {
        SortBy::increasing_or_decreasing()
    }

    fn null_handling(&self) -> NullHandling {
        NullHandling::Last
    }
}

impl Purchase {
    /// Helper function for load_prime_ministers to create a new Purchase
    fn new(
        id: usize,
        name: &'static str,
        description: &'static str,
        price: f64,
        quantity: usize,
    ) -> Purchase {
        Purchase {
            name: name.to_string(),
            description: description.to_string(),
            id,
            price,
            quantity,
        }
    }
}

/// Our mock data source. In a real app this could be something like a `reqwest` call
async fn load_prime_ministers() -> Vec<Purchase> {
    vec![
        Purchase::new(1, "Apple", "iPhone 12", 799.0, 1),
        Purchase::new(2, "Apple", "iPhone 12 Pro", 999.0, 1),
    ]
}