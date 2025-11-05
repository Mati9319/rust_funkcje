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

) -> Wynik {
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
        zaokr(brutto_zus_zdr_pod
        + brutto_zdr_pod
        + brutto_pod
        + brutto_netto
        + brutto_nie_zus_zdr_pod
        + brutto_nie_zdr_pod
        + brutto_nie_pod
        + brutto_nie_netto);

    let brutto_wyp =    //cały wypłacany przychód
        zaokr(brutto_zus_zdr_pod
        + brutto_zdr_pod
        + brutto_pod
        + brutto_netto);

    let pd_zus =        //przychód stanowiący podstawę składek ZUS
        zaokr(brutto_zus_zdr_pod
        + brutto_nie_zus_zdr_pod);

    let pd_zdr =        //przychód stanowiący podstawę składki zdrowotnej
        zaokr(brutto_zus_zdr_pod
        + brutto_zdr_pod
        + brutto_nie_zus_zdr_pod
        + brutto_nie_zdr_pod);

    let pd_pod =        //przychód stanowiący podstawę podatku
        zaokr(brutto_zus_zdr_pod
        + brutto_zdr_pod
        + brutto_pod
        + brutto_nie_zus_zdr_pod
        + brutto_nie_zdr_pod
        + brutto_nie_pod);

    let zus_emerytalna = zaokr(pd_zus * 0.0976);
    let zus_rentowa = zaokr(pd_zus * 0.015);
    let zus_chorobowa = zaokr(pd_zus * 0.0245);

    let zus = zaokr(zus_emerytalna + zus_rentowa + zus_chorobowa); //suma składek ZUS

    let zdrowotna = zaokr((pd_zdr - zus) * 0.09);

    let podatek = if pod_zwol {
        0.0
    } else {
        ((pd_pod - zus - kup).round() * pod_proc - ulga).round()
    };

    let netto = zaokr(brutto_wyp - zus - zdrowotna - podatek - potr_dod);

    Wynik {
        brutto_cal,
        brutto_wyp,
        zus_emerytalna,
        zus_rentowa,
        zus_chorobowa,
        zdrowotna,
        podatek,
        netto
    }
}

pub struct Wynik {
    brutto_cal: f32,
    brutto_wyp: f32,
    zus_emerytalna: f32,
    zus_rentowa: f32,
    zus_chorobowa: f32,
    zdrowotna: f32,
    podatek: f32,
    netto: f32
}

impl Wynik {
    pub fn show(&self) {
        println!(
"Całkowite wynagrodzenie brutto:      {:.2} zł
Wypłacane wynagrodzenie brutto:      {:.2} zł

Składka na ubezpieczenie emerytalne: {:.2} zł
Składka na ubezpieczenie rentowe:    {:.2} zł
Składka na ubezpieczenie chorobowe:  {:.2} zł

Składka na ubezpieczenie zdrowotne:  {:.2} zł

Zaliczka na podatek dochodowy:       {:.2} zł

Wypłacane wynagrodzenie netto:       {:.2} zł",
        self.brutto_cal,
        self.brutto_wyp,
        self.zus_emerytalna,
        self.zus_rentowa,
        self.zus_chorobowa,
        self.zdrowotna,
        self.podatek,
        self.netto);
    }

    pub fn wiecej(&mut self, mnoznik: f32) {
        self.netto = self.netto * mnoznik;
    }
}
