use crate::error::*;


#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ScreenRange {
    Max(usize),
}

#[derive(Debug, Clone)]
pub struct ScreenArea {
    maxx : ScreenRange,
    maxy : ScreenRange,
}


impl Default for ScreenArea {
    fn default() -> Self {
        ScreenArea { maxx: ScreenRange::Max(0usize), maxy: ScreenRange::Max(0usize) }
    }
}

impl ScreenArea {
    pub fn new(x : usize, y: usize) -> Self {
        ScreenArea {
            maxx: ScreenRange::Max(x),
            maxy: ScreenRange::Max(y),
        }
    }

    pub fn from(range : & str) -> Result<ScreenArea> {
        ScreenArea::parse(range)
    }   
    // X:Y
    pub fn parse(range_raw : &str) -> Result<ScreenArea> {
        let mut new_range = ScreenArea::default();
        let mut range_iter = range_raw.bytes();
        let fist_byte = range_iter.next().ok_or("Emmpty MaxX:MaxY")?;

        if fist_byte == b':' {
            if range_iter.next() == Some(b'-') {
                // -3 :)
                new_range.maxy = ScreenRange::Max(0usize)
            } else {
                let value = range_raw[1..].parse()?;
                new_range.maxy = ScreenRange::Max(value)
            }
            return Ok(new_range);
        } else if range_raw.bytes().last().ok_or("Empty MaxX:MaxY")? == b':' {
            if fist_byte == b'-' {
                new_range.maxy = ScreenRange::Max(0usize)
            } else {
                let value = range_raw[1..range_raw.len()-1].parse()?;
                new_range.maxy = ScreenRange::Max(value)
            }
            return Ok(new_range)            
        } 

        let line_numbers : Vec<&str> = range_raw.split(':').collect();
        match line_numbers.len() {
            1 => {                
                new_range.maxx = ScreenRange::Max(line_numbers[0].parse()?);
                new_range.maxy = ScreenRange::Max(line_numbers[0].parse()?);
                Ok(new_range)
            }
            2 => {
                new_range.maxx = ScreenRange::Max(line_numbers[0].parse()?);
                new_range.maxy = ScreenRange::Max(line_numbers[0].parse()?);
                Ok(new_range)                
            } 
            _ => Err("Unable to parse".into()),
        }
    } 
}

