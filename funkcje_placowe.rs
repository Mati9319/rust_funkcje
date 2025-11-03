fn zaokr(x: f32) -> f32 {
    (x * 100.0).round() / 100.0
}

pub fn brutto_na_netto(
    brutto: f32,
    pod_zwol: bool, //czy zwolnienie z podatku
    jakie_kup: char, //'0' <- brak kosztów, '1' <- 250, '2' <- 300
    jaki_pod: char, //'1' <- 12%, '2' <- 32%
    jaka_ulga: char //'0' <- brak ulgi, '1' <- 300, '2' <- 150, '3' <- 100
    
) {
    let kup = match jakie_kup {
        '_' => 0.0,
        '1' => 250.0,
        '2' => 300.0
    };

    let pod = match jaki_pod {
        '_' => 0.12,
        '2' => 0.32
    }

    let ulga = match jaka_ulga {
        '_' => 0.0,
        '1' => 300.0,
        '2' => 150.0,
        '3' => 100.0
    }
    
    let zus_emerytalna = zaokr(brutto * 0.0976);
    let zus_rentowa = zaokr(brutto * 0.015);
    let zus_chorobowa = zaokr(brutto * 0.0245);

    let zus = zus_emerytalna + zus_rentowa + zus_chorobowa;

    let zdrowotna = zaokr((brutto - zus) * 0.09);

    let podatek = if pod_zwol {
        0.0
    } else {
        ((brutto - zus - kup).round() * pod - ulga).round()
    };

    let netto = brutto - zus - zdrowotna - podatek;

    let wynik = [zus_emerytalna, zus_rentowa, zus_chorobowa, zdrowotna, podatek, netto];

    wynik
}
