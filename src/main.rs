use testowanie::lista_plac::*;

fn main() {
    let mut x: ListaPlac = ListaPlac::nalicz(5300.0+262.96+52.08+863.52+78.4+50.0+31.92+64.12+46.2+9.8+92.68+689.0+9.8+164.0+1457.0+249.76+795.0+1754.64+274.68+225.22+44.8, 0.0, 0.0, 625.0+441.0, 100.0, 0.0, 0.0, 0.0, 20.2+500.0+300.0+53.0, false, '2', '1', '1');
    x.wyswietl();
    x.jaka_ulga = '2';
    x.przelicz_i_wyswietl();
    x.pod_zwol = true;
    x.przelicz_i_wyswietl();
}
