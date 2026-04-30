use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

use crate::vec3::Point3;

/// The geometry extracted from a PLY file.
pub struct PlyMeshData {
    pub positions: Vec<Point3>,
    pub indices: Vec<[u32; 3]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlyFormat { Ascii, BinaryLE, BinaryBE }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScalarType { Float, Double, Char, UChar, Short, UShort, Int, UInt }

impl ScalarType {
    fn parse(s: &str) -> io::Result<Self> {
        Ok(match s {
            "float"   | "float32" => ScalarType::Float,
            "double"  | "float64" => ScalarType::Double,
            "char"    | "int8"    => ScalarType::Char,
            "uchar"   | "uint8"   => ScalarType::UChar,
            "short"   | "int16"   => ScalarType::Short,
            "ushort"  | "uint16"  => ScalarType::UShort,
            "int"     | "int32"   => ScalarType::Int,
            "uint"    | "uint32"  => ScalarType::UInt,
            other => return Err(Error::new(ErrorKind::InvalidData,
                format!("Unsupported PLY scalar type: {}", other))),
        })
    }
}

#[derive(Debug, Clone)]
enum Property {
    Scalar { name: String, ty: ScalarType },
    List   { name: String, count_ty: ScalarType, item_ty: ScalarType },
}

#[derive(Debug, Clone)]
struct Element {
    name: String,
    count: usize,
    properties: Vec<Property>,
}

struct Header {
    format: PlyFormat,
    elements: Vec<Element>,
}

/// Load a PLY file from a give path.
pub fn load(path: impl AsRef<Path>) -> io::Result<PlyMeshData> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Parse the header
    let header = parse_header(&mut reader)?;

    match header.format {
        PlyFormat::Ascii => parse_ascii_body(&mut reader, &header),
        PlyFormat::BinaryLE | PlyFormat::BinaryBE => Err(Error::new(
            ErrorKind::Unsupported,
            "Binary PLY not implemented yet (TODO)",
        )),
    }
}

// ---------------- Header ----------------

fn parse_header<R: BufRead>(reader: &mut R) -> io::Result<Header> {
    let mut line = String::new();
    let mut read_line = |buf: &mut String| -> io::Result<()> {
        buf.clear();
        let n = reader.read_line(buf)?;
        if n == 0 { return Err(Error::new(ErrorKind::UnexpectedEof, "EOF in PLY header")); }
        Ok(())
    };

    read_line(&mut line)?;
    if line.trim() != "ply" {
        return Err(Error::new(ErrorKind::InvalidData, "File does not start with 'ply'"));
    }

    let mut format: Option<PlyFormat> = None;
    let mut elements: Vec<Element> = Vec::new();

    loop {
        read_line(&mut line)?;
        let trimmed = line.trim();
        if trimmed == "end_header" { break; }
        if trimmed.is_empty() || trimmed.starts_with("comment") { continue; }

        let mut tokens = trimmed.split_whitespace();
        match tokens.next() {
            Some("format") => {
                let kind = tokens.next().unwrap_or("");
                format = Some(match kind {
                    "ascii" => PlyFormat::Ascii,
                    "binary_little_endian" => PlyFormat::BinaryLE,
                    "binary_big_endian" => PlyFormat::BinaryBE,
                    other => return Err(Error::new(ErrorKind::InvalidData,
                        format!("Unsupported PLY format: {}", other))),
                });
            }
            Some("element") => {
                let name = tokens.next().unwrap_or("").to_string();
                let count: usize = tokens.next().unwrap_or("0").parse()
                    .map_err(|_| Error::new(ErrorKind::InvalidData, "bad element count"))?;
                elements.push(Element { name, count, properties: Vec::new() });
            }
            Some("property") => {
                let elem = elements.last_mut().ok_or_else(||
                    Error::new(ErrorKind::InvalidData, "property before element"))?;
                let prop = parse_property_line(&mut tokens)?;
                elem.properties.push(prop);
            }
            _ => {} // Ignore known header lines
        }
    }

    let format = format.ok_or_else(||
        Error::new(ErrorKind::InvalidData, "missing PLY format line"))?;

    Ok(Header { format, elements })
}

fn parse_property_line<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> io::Result<Property> {
    let first = tokens.next().unwrap_or("");
    if first == "list" {
        let count_ty = ScalarType::parse(tokens.next().unwrap_or(""))?;
        let item_ty = ScalarType::parse(tokens.next().unwrap_or(""))?;
        let name = tokens.next().unwrap_or("").to_string();
        Ok(Property::List { name, count_ty, item_ty })
    } else {
        let ty = ScalarType::parse(first)?;
        let name = tokens.next().unwrap_or("").to_string();
        Ok(Property::Scalar { name, ty })
    }
}

// ---------------- ASCII Body ----------------

fn parse_ascii_body<R: BufRead>(reader: &mut R, header: &Header) -> io::Result<PlyMeshData> {
    // Locate the vertex and face elements; ignore any others
    let mut positions: Vec<Point3> = Vec::new();
    let mut indices: Vec<[u32; 3]> = Vec::new();

    // Buffer reused across lines
    let mut line = String::new();

    for element in &header.elements {
        match element.name.as_str() {
            "vertex" => {
                positions.reserve_exact(element.count);
                let (x_pos, y_pos, z_pos) = locate_xyz(&element.properties)?;
                let prop_count = element.properties.len();

                for _ in 0..element.count {
                    line.clear();
                    let n = reader.read_line(&mut line)?;
                    if n == 0 { return Err(Error::new(ErrorKind::UnexpectedEof, "EOF in vertex data")); }
                    let mut fields = line.split_whitespace();
                    let mut vals = [0.0f64; 3];
                    let mut have = [false; 3];
                    for i in 0..prop_count {
                        let tok = fields.next().ok_or_else(||
                            Error::new(ErrorKind::InvalidData, "short vertex line"))?;
                        if i == x_pos { vals[0] = parse_f64(tok)?; have[0] = true; }
                        else if i == y_pos { vals[1] = parse_f64(tok)?; have[1] = true; }
                        else if i == z_pos { vals[2] = parse_f64(tok)?; have[2] = true; }
                        // else: skip token (e.g. r/g/b/normal)
                    }
                    if !have[0] || !have[1] || !have[2] {
                        return Err(Error::new(ErrorKind::InvalidData, "missing x/y/z on vertex"));
                    }
                    positions.push(Point3::new(vals[0], vals[1], vals[2]));
                }
            }
            "face" => {
                indices.reserve(element.count); // can grow if there is fan triangulation
                for _ in 0..element.count {
                    line.clear();
                    let n = reader.read_line(&mut line)?;
                    if n == 0 { return Err(Error::new(ErrorKind::UnexpectedEof, "EOF in face data")); }
                    
                    let mut fields = line.split_whitespace();
                    // First number is the vertex count for this face
                    let n_verts: usize = fields.next()
                        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "empty face line"))?
                        .parse().map_err(|_| Error::new(ErrorKind::InvalidData, "bad face count"))?;
                    if n_verts < 3 { continue; }

                    // Read all vertex indices.
                    let mut verts: Vec<u32> = Vec::with_capacity(n_verts);
                    for _ in 0..n_verts {
                        let tok = fields.next()
                            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "short face line"))?;
                        verts.push(tok.parse().map_err(|_|
                            Error::new(ErrorKind::InvalidData, "bad face index"))?);
                    }

                    // Fan triangulation if necessary
                    for k in 1..n_verts-1 {
                        indices.push([verts[0], verts[k], verts[k+1]]);
                    }
                }
            }
            _ => {
                // Unknown element: consume `count` lines.
                for _ in 0..element.count {
                    line.clear();
                    reader.read_line(&mut line)?; 
                }
            }
        }
    }

    println!("Loaded mesh: {} vertices, {} faces", positions.len(), indices.len());

    Ok(PlyMeshData { positions, indices })
}

fn locate_xyz(props: &[Property]) -> io::Result<(usize, usize, usize)> {
    let mut x = None; let mut y = None; let mut z = None;
    for (i, p) in props.iter().enumerate() {
        if let Property::Scalar { name, .. } = p {
            match name.as_str() {
                "x" => x = Some(i),
                "y" => y = Some(i),
                "z" => z = Some(i),
                _ => {}
            }
        }
    }
    match (x, y, z) {
        (Some(x), Some(y), Some(z)) => Ok((x, y, z)),
        _ => Err(Error::new(ErrorKind::InvalidData, "vertex element missing x/y/z properties")),
    }
}

#[inline]
fn parse_f64(s: &str) -> io::Result<f64> {
    s.parse().map_err(|_|
        Error::new(ErrorKind::InvalidData, format!("bad float: {s}")))
}