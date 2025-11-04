fn zaokr(x: f32) -> f32 {
    (x * 100.0).round() / 100.0
}

pub fn brutto_na_netto(
    brutto_zus_zdr_pod: f32,        //przychód ozusowany, ozdrowotniony i opodatkowany
    brutto_zdr_pod: f32,            //przychód ozdrowotniony i opodatkowany
    brutto_pod: f32,                //przychód opodatkowany
    brutto_netto: f32,              //przychód bez potrąceń
    brutto_nie_zus_zdr_pod: f32,    //przychód niewypłacany (ozusowany, ozdrowotniony i opodatkowany)
    brutto_nie_zdr_pod: f32,        //przychód niewypłacany (ozdrowotniony i opodatkowany)
    brutto_nie_pod: f32,            //przychód niewypłacany (opodatkowany)
    brutto_nie_netto: f32,          //przychód niewypłacany (bez potrąceń)
    potr_dod: f32,                  //dodatkowe potrącenia
    pod_zwol: bool,                 //czy zwolnienie z podatku
    jakie_kup: char,                //'0' <- brak kosztów, '1' <- 250, '2' <- 300
    jaki_pod_proc: char,            //'1' <- 12%, '2' <- 32%
    jaka_ulga: char                 //'0' <- brak ulgi, '1' <- 300, '2' <- 150, '3' <- 100

) -> [f32; 8] {
    let kup = match jakie_kup {
        '1' => 250.0,
        '2' => 300.0,
        _ => 0.0
    };

    let pod_proc = match jaki_pod_proc {
        '2' => 0.32,
        _ => 0.12
    };

    let ulga = match jaka_ulga {
        '1' => 300.0,
        '2' => 150.0,
        '3' => 100.0,
        _ => 0.0
    };

    let brutto_cal =    //cały przychód
        brutto_zus_zdr_pod
        + brutto_zdr_pod
        + brutto_pod
        + brutto_netto
        + brutto_nie_zus_zdr_pod
        + brutto_nie_zdr_pod
        + brutto_nie_pod
        + brutto_nie_netto;

    let brutto_wyp =    //cały wypłacany przychód
        brutto_zus_zdr_pod
        + brutto_zdr_pod
        + brutto_pod
        + brutto_netto;

    let pd_zus =        //przychód stanowiący podstawę składek ZUS
        brutto_zus_zdr_pod
        + brutto_nie_zus_zdr_pod;

    let pd_zdr =        //przychód stanowiący podstawę składki zdrowotnej
        brutto_zus_zdr_pod
        + brutto_nie_zus_zdr_pod
        + brutto_nie_zdr_pod;

    let pd_pod =        //przychód stanowiący podstawę podatku
        brutto_zus_zdr_pod
        + brutto_zdr_pod
        + brutto_pod
        + brutto_nie_zus_zdr_pod
        + brutto_nie_zdr_pod
        + brutto_nie_pod;

    let zus_emerytalna = zaokr(pd_zus * 0.0976);
    let zus_rentowa = zaokr(pd_zus * 0.015);
    let zus_chorobowa = zaokr(pd_zus * 0.0245);

    let zus = zus_emerytalna + zus_rentowa + zus_chorobowa; //suma składek ZUS

    let zdrowotna = zaokr((pd_zdr - zus) * 0.09);

    let podatek = if pod_zwol {
        0.0
    } else {
        ((pd_pod - zus - kup).round() * pod_proc - ulga).round()
    };

    let netto = brutto_wyp - zus - zdrowotna - podatek - potr_dod;

    [brutto_cal,
    brutto_wyp,
    zus_emerytalna,
    zus_rentowa,
    zus_chorobowa,
    zdrowotna,
    podatek,
    netto]
}
