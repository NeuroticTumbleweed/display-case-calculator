use std::fs;

use clap::Parser;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long)]
    width: usize,
    #[clap(short, long)]
    height: usize,
    #[clap(short, long)]
    depth: usize,
    #[clap(short, long, default_value_t = 1)]
    perspex_thickness: usize,
    #[clap(long, default_value_t = 1)]
    wood_thickness: usize,
}

#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

fn main() {
    let args: Args = Args::parse();

    println!("{:?}", &args);
    let mut perspex_plates: Vec<&Rectangle> = Vec::new();
    let mut wood_plates: Vec<&Rectangle> = Vec::new();
    // face plate
    let face_plate: Rectangle = Rectangle {
        // The inner width of the display case
        // + 2 times the thickness of the perspex material (for each side wall).
        width: args.width + 2 * args.perspex_thickness,
        // The inner height of the display case
        // + the thickness of the perspex material for the top plate
        // + the thickness of wood for the inner wood base piece.
        height: args.height + args.perspex_thickness + args.wood_thickness,
    };
    println!("Face plate: {:?}", face_plate);
    perspex_plates.push(&face_plate);

    // back plate
    let back_plate: Rectangle = Rectangle {
        // The inner width of the display case
        // + 2 times the thickness of the perspex material (for each side wall).
        width: args.width + 2 * args.perspex_thickness,
        // The inner height of the display case
        // + the thickness of the perspex material for the top plate
        // + the thickness of wood for the inner wood base piece.
        height: args.height + args.perspex_thickness + args.wood_thickness,
    };
    println!("Back plate: {:?}", back_plate);
    wood_plates.push(&back_plate);

    // side plates
    let side_plate: Rectangle = Rectangle {
        // The inner depth of the display case.
        width: args.depth,
        // The inner height of the display case
        // + the thickness of wood for the inner wood base piece.
        height: args.height + args.wood_thickness,
    };
    println!("Side plates: {:?}", side_plate);
    perspex_plates.push(&side_plate);
    perspex_plates.push(&side_plate);

    // top plate
    let top_plate: Rectangle = Rectangle {
        // The inner width of the display case
        // + 2 times the thickness of the perspex material (for each side wall).
        width: args.width + 2 * args.perspex_thickness,
        height: args.depth,
    };
    println!("Top plate: {:?}", top_plate);
    perspex_plates.push(&top_plate);

    // inner base plate
    let inner_base_plate: Rectangle = Rectangle {
        // The inner width of the display case.
        width: args.width,
        // The inner depth of the display case.
        height: args.depth,
    };
    println!("Inner base plate: {:?}", inner_base_plate);
    wood_plates.push(&inner_base_plate);

    // actual base plate
    let actual_base_plate: Rectangle = Rectangle {
        // The inner width of the display case
        // + 2 times the thickness of the perspex material (for each side wall).
        width: args.width + 2 * args.perspex_thickness,
        // The inner width of the display case
        // + thickness of the perspex material for the face plate
        // + thickness of the wood material for the back plate.
        height: args.depth + args.perspex_thickness + args.wood_thickness,
    };
    println!("Actual base plate: {:?}", actual_base_plate);
    wood_plates.push(&actual_base_plate);
    fs::create_dir_all("plates").unwrap();
    write_out_to_svg(&perspex_plates, "plates/perspex.svg");
    write_out_to_svg(&wood_plates, "plates/wood.svg");
}

fn write_out_to_svg(rectangles: &Vec<&Rectangle>, file_name: &str) {
    let mut document = Document::new();

    let mut move_coords = (10, 10);

    let mut max_vert = 0;
    let mut max_horz = 0;

    let mut total_height = 10;
    let mut total_width = 10;

    for (index, rectangle) in rectangles.iter().enumerate() {
        println!("{:?}", rectangle);

        let data = Data::new()
            .move_by(move_coords)
            .line_by((0, rectangle.height))
            .line_by((rectangle.width, 0))
            .line_by((0, -isize::try_from(rectangle.height).unwrap()))
            .close();

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.05)
            .set("d", data);

        document = document.add(path);
        if rectangle.height > max_vert {
            max_vert = rectangle.height;
        }
        if (index + 1) % 4 == 0 {
            move_coords = (10, max_vert + move_coords.1);
            total_height += max_vert;
            max_vert = 0;
            total_width += rectangle.width;

            if total_width > max_horz {
                max_horz = total_width;
            }
            total_width = 10;
        } else {
            move_coords = (move_coords.0 + rectangle.width, move_coords.1);
            total_width += rectangle.width;
        }
    }
    if total_width > max_horz {
        max_horz = total_width;
    }
    total_height += max_vert;
    document = document
        .set("viewBox", (0, 0, max_horz + 10, total_height + 10))
        .set("width", format!("{}mm", max_horz + 10))
        .set("height", format!("{}mm", total_height + 10));

    svg::save(file_name, &document).unwrap();
}
