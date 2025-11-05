pub fn zaokr(x: f32) -> f32 {
    (x * 100.0).round() / 100.0
}

pub struct ListaPlac {
    brutto_zus_zdr_pod: f32,
    brutto_zdr_pod: f32,
    brutto_pod: f32,
    brutto_netto: f32,              
    brutto_nie_zus_zdr_pod: f32,    
    brutto_nie_zdr_pod: f32,        
    brutto_nie_pod: f32,           
    brutto_nie_netto: f32,         
    potr_dod: f32,                 
    pod_zwol: bool,               
    jakie_kup: char,                
    jaki_pod_proc: char,            
    jaka_ulga: char,            
    brutto_cal: f32,
    brutto_wyp: f32,
    pd_zus: f32,
    pd_zdr: f32,
    pd_pod: f32,
    zus_emerytalna: f32,
    zus_rentowa: f32,
    zus_chorobowa: f32,
    zus: f32,
    zdrowotna: f32,
    podatek: f32,
    netto: f32
}

impl ListaPlac {
    pub fn nalicz(
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

    ) -> Self {
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
    
        Self {
            brutto_zus_zdr_pod,
            brutto_zdr_pod,
            brutto_pod,
            brutto_netto,              
            brutto_nie_zus_zdr_pod,    
            brutto_nie_zdr_pod,        
            brutto_nie_pod,           
            brutto_nie_netto,         
            potr_dod,                 
            pod_zwol,               
            jakie_kup,                
            jaki_pod_proc,            
            jaka_ulga,            
            brutto_cal,
            brutto_wyp,
            pd_zus,
            pd_zdr,
            pd_pod,
            zus_emerytalna,
            zus_rentowa,
            zus_chorobowa,
            zus,
            zdrowotna,
            podatek,
            netto
        }
    }

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

    pub fn przelicz(&mut self) {
        self = Self::nalicz(
            self.brutto_zus_zdr_pod,
            self.brutto_zdr_pod,
            self.brutto_pod,
            self.brutto_netto,
            self.brutto_nie_zus_zdr_pod,
            self.brutto_nie_zdr_pod,
            self.brutto_nie_pod,
            self.brutto_nie_netto,
            self.potr_dod,
            self.pod_zwol,
            self.jakie_kup,
            self.jaki_pod_proc,
            self.jaka_ulga)
    }
}
