mod funkcje_placowe;
use funkcje_plackskakowe::*;

fn main() {
    let x = brutto_na_netto(6500.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false, '2', '1', '1');
    x.show();
}
