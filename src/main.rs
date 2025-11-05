mod funkcje_placowe;
use funkcje_placowe::*;

fn main() {
    let x = brutto_na_netto(8940.2, 0.0, 0.0, 625.0, 100.0, 0.0, 0.0, 0.0, 878.7, false, '2', '1', '1');
    x.show();
}
