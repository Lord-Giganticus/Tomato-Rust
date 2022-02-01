extern "C" {
    fn BMGToXML(bmgpath: *const u8, tblpath: *const u8) -> bool;
    fn XMLToBMG(xmlpath: *const u8, littleendian: bool) -> bool;
}

pub fn bmgtoxml(bmgpath: &str, tblpath: &str) -> bool {
    unsafe {
        return BMGToXML(bmgpath.as_ptr(), tblpath.as_ptr());
    }
}

pub fn xmltobmg(xmlpath: &str, littleendian: bool) -> bool {
    unsafe {
        return XMLToBMG(xmlpath.as_ptr(), littleendian);
    }
}