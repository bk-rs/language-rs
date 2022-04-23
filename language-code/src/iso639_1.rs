//! [List of ISO 639-1 codes - Wikipedia](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes)

use country_code::iso3166_1::alpha_2::CountryCode;

use crate::{language_code, language_tag};

language_code! {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[allow(non_camel_case_types)]
    pub enum LanguageCode {
        ab,
        aa,
        af,
        ak,
        sq,
        am,
        ar,
        an,
        hy,
        r#as,
        av,
        ae,
        ay,
        az,
        bm,
        ba,
        eu,
        be,
        bn,
        bi,
        bs,
        br,
        bg,
        my,
        ca,
        ch,
        ce,
        ny,
        zh,
        cu,
        cv,
        kw,
        co,
        cr,
        hr,
        cs,
        da,
        dv,
        nl,
        dz,
        en,
        eo,
        et,
        ee,
        fo,
        fj,
        fi,
        fr,
        fy,
        ff,
        gd,
        gl,
        lg,
        ka,
        de,
        el,
        kl,
        gn,
        gu,
        ht,
        ha,
        he,
        hz,
        hi,
        ho,
        hu,
        is,
        io,
        ig,
        id,
        ia,
        ie,
        iu,
        ik,
        ga,
        it,
        ja,
        jv,
        kn,
        kr,
        ks,
        kk,
        km,
        ki,
        rw,
        ky,
        kv,
        kg,
        ko,
        kj,
        ku,
        lo,
        la,
        lv,
        li,
        ln,
        lt,
        lu,
        lb,
        mk,
        mg,
        ms,
        ml,
        mt,
        gv,
        mi,
        mr,
        mh,
        mn,
        na,
        nv,
        nd,
        nr,
        ng,
        ne,
        no,
        nb,
        nn,
        ii,
        oc,
        oj,
        or,
        om,
        os,
        pi,
        ps,
        fa,
        pl,
        pt,
        pa,
        qu,
        ro,
        rm,
        rn,
        ru,
        se,
        sm,
        sg,
        sa,
        sc,
        sr,
        sn,
        sd,
        si,
        sk,
        sl,
        so,
        st,
        es,
        su,
        sw,
        ss,
        sv,
        tl,
        ty,
        tg,
        ta,
        tt,
        te,
        th,
        bo,
        ti,
        to,
        ts,
        tn,
        tr,
        tk,
        tw,
        ug,
        uk,
        ur,
        uz,
        ve,
        vi,
        vo,
        wa,
        cy,
        wo,
        xh,
        yi,
        yo,
        za,
        zu,
    }
}

language_tag! {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LanguageTag {
        pub language_code: LanguageCode,
        pub country_code: Option<CountryCode>,
    }
}

impl Default for LanguageTag {
    fn default() -> Self {
        Self::new(LanguageCode::en, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::ToString as _;

    use csv::Reader;

    #[test]
    fn test_language_code() {
        let mut rdr = Reader::from_reader(
            include_str!("../tests/List_of_ISO_639-1_codes/list.csv").as_bytes(),
        );

        let mut n = 0;
        for record in rdr.records() {
            let record = record.unwrap();

            let code = &record[1];
            assert_eq!(code.parse::<LanguageCode>().unwrap().to_string(), code);
            n += 1;
        }

        assert_eq!(LanguageCode::VARS.len(), n);
    }

    #[test]
    fn test_language_tag() {
        assert_eq!(
            "en".parse::<LanguageTag>().unwrap(),
            LanguageTag::new(LanguageCode::en, None)
        );
        assert_eq!(LanguageTag::new(LanguageCode::en, None).to_string(), "en",);

        assert_eq!(
            "zh-CN".parse::<LanguageTag>().unwrap(),
            LanguageTag::new(LanguageCode::zh, Some(CountryCode::CN))
        );
        assert_eq!(
            LanguageTag::new(LanguageCode::zh, Some(CountryCode::CN)).to_string(),
            "zh-CN"
        );
        assert_eq!(
            "zh-TW".parse::<LanguageTag>().unwrap(),
            LanguageTag::new(LanguageCode::zh, Some(CountryCode::TW))
        );
    }
}
