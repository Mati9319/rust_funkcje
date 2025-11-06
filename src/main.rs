use testowanie::lista_plac::*;

fn main() {
    let mut x = ListaPlac::nalicz(2644.58, 2105.76, 1691.4, 72.0, 100.0, 0.0, 0.0, 0.0, 550.0, false, '2', '1', '1');
    x.wyswietl();
    x.pod_zwol = true;
    x.brutto_zus_zdr_pod += 1000.00;
    x.przelicz();
    x.wyswietl();
}
