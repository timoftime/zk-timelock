use crate::yt6_776::{Fq, Fq3, Fq3Config};
use ark_ff::{
    fields::fp6_2over3::{Fp6, Fp6Config},
    Field, MontFp,
};

pub type Fq6 = Fp6<Fq6Config>;

pub struct Fq6Config;

impl Fp6Config for Fq6Config {
    type Fp3Config = Fq3Config;

    /// NONRESIDUE = (0, 1, 0)
    #[rustfmt::skip]
    const NONRESIDUE: Fq3 = Fq3::new(Fq::ZERO, Fq::ONE, Fq::ZERO);

    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP6_C1: &'static [Fq] = &[
        MontFp!("1"),
        MontFp!("142530150333094371869703769686731425964913314771299981144308427540645932934092011898942172819751386393848850418097664734615766850620581003523120817544498576459207635586430646422648400528506404609884695553029564408157616901335302173986"),
        MontFp!("142530150333094371869703769686731425964913314771299981144308427540645932934092011898942172819751386393848850418097664734615766850620581003523120817544498576459207635586430646422648400528506404609884695553029564408157616901335302173985"),
        MontFp!("-1"),
        MontFp!("160346419124731168354354950401762388232771363404217916502074571949364243759857652128488749182853891605869079242021400757430774352434516395222551724622105600641910619996330766274708685150723349823386207315893156041672692769501110498977"),
        MontFp!("160346419124731168354354950401762388232771363404217916502074571949364243759857652128488749182853891605869079242021400757430774352434516395222551724622105600641910619996330766274708685150723349823386207315893156041672692769501110498978"),
    ];
}
