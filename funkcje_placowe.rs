fn zaokr(x: f32) -> f32 {
    (x * 100.0).round() / 100.0
}

pub fn brutto_na_netto(brutto: f32) -> f32 {
    let zus_emerytalna = zaokr(brutto * 0.0976);
    let zus_rentowa = zaokr(brutto * 0.015);
    let zus_chorobowa = zaokr(brutto * 0.0245);

    let zus = zus_emerytalna + zus_rentowa + zus_chorobowa;

    let zdrowotna = zaokr((brutto - zus) * 0.09);

    let podatek = ((brutto - zus - 250.0).round() * 0.12 - 300.0).round();

    let netto = brutto - zus - zdrowotna - podatek;

    /*println!("Brutto: {:.2} zł", brutto);
    println!();
    println!("Składka emerytalna: {:.2} zł", zus_emerytalna);
    println!("Składka rentowa: {:.2} zł", zus_rentowa);
    println!("Składka chorobowa: {:.2} zł", zus_chorobowa);
    println!();
    println!("Składka zdrowotna: {:.2} zł", zdrowotna);
    println!();
    println!("Podatek: {:.2} zł", podatek);
    println!();
    println!("Netto: {:.2} zł", netto);*/

    netto
}
