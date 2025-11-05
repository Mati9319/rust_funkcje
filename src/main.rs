mod funkcje_placowe;
use funkcje_placowe::*;

fn main() {
    let x = ListaPlac::nalicz(2644.58, 2105.76, 1691.4, 72.0, 100.0, 0.0, 0.0, 0.0, 550.0, false, '2', '1', '1');
    x.show();
}
