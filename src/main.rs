mod funkcje_placowe;
use funkcje_placowe::*;

fn main() {
    let x = brutto_na_netto(2594.58, 2105.76, 1691.4, 122.0, 100.0, 0.0, 0.0, 0.0, 550.0, false, '2', '1', '1');
    x.show();
}
