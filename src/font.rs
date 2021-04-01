// MIT License
//
// Copyright (c) 2020 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub struct FFont<'a> {
    a: Vec<&'a str>,
    b: Vec<&'a str>,
    c: Vec<&'a str>,
    d: Vec<&'a str>,
    e: Vec<&'a str>,
    f: Vec<&'a str>,
    g: Vec<&'a str>,
    h: Vec<&'a str>,
    i: Vec<&'a str>,
    j: Vec<&'a str>,
    k: Vec<&'a str>,
    l: Vec<&'a str>,
    m: Vec<&'a str>,
    n: Vec<&'a str>,
    o: Vec<&'a str>,
    p: Vec<&'a str>,
    q: Vec<&'a str>,
    r: Vec<&'a str>,
    s: Vec<&'a str>,
    t: Vec<&'a str>,
    u: Vec<&'a str>,
    v: Vec<&'a str>,
    w: Vec<&'a str>,
    x: Vec<&'a str>,
    y: Vec<&'a str>,
    z: Vec<&'a str>,

    dot   : Vec<&'a str>,
    comma : Vec<&'a str>,
    exclam: Vec<&'a str>,

    one  : Vec<&'a str>,
    two  : Vec<&'a str>,
    three: Vec<&'a str>,
    four : Vec<&'a str>,
    five : Vec<&'a str>,
    six  : Vec<&'a str>,
    seven: Vec<&'a str>,
    eight: Vec<&'a str>,
    nine : Vec<&'a str>,
    zero : Vec<&'a str>
}

impl FFont<'_> {
    pub fn generate(font: &FFont, data: String) -> String {
        let mut generated     = String::new();
        let mut _index    : usize    = 0;

        // _index = character width
        while _index != 5 {
            for character in data.chars() {
                match character {
                    'a' | 'A' => generated.push_str(font.a.get(_index).unwrap()),
                    'b' | 'B' => generated.push_str(font.b.get(_index).unwrap()),
                    'c' | 'C' => generated.push_str(font.c.get(_index).unwrap()),
                    'd' | 'D' => generated.push_str(font.d.get(_index).unwrap()),
                    'e' | 'E' => generated.push_str(font.e.get(_index).unwrap()),
                    'f' | 'F' => generated.push_str(font.f.get(_index).unwrap()),
                    'g' | 'G' => generated.push_str(font.g.get(_index).unwrap()),
                    'h' | 'H' => generated.push_str(font.h.get(_index).unwrap()),
                    'i' | 'I' => generated.push_str(font.i.get(_index).unwrap()),
                    'j' | 'J' => generated.push_str(font.j.get(_index).unwrap()),
                    'k' | 'K' => generated.push_str(font.k.get(_index).unwrap()),
                    'l' | 'L' => generated.push_str(font.l.get(_index).unwrap()),
                    'm' | 'M' => generated.push_str(font.m.get(_index).unwrap()),
                    'n' | 'N' => generated.push_str(font.n.get(_index).unwrap()),
                    'o' | 'O' => generated.push_str(font.o.get(_index).unwrap()),
                    'p' | 'P' => generated.push_str(font.p.get(_index).unwrap()),
                    'q' | 'Q' => generated.push_str(font.q.get(_index).unwrap()),
                    'r' | 'R' => generated.push_str(font.r.get(_index).unwrap()),
                    's' | 'S' => generated.push_str(font.s.get(_index).unwrap()),
                    't' | 'T' => generated.push_str(font.t.get(_index).unwrap()),
                    'u' | 'U' => generated.push_str(font.u.get(_index).unwrap()),
                    'v' | 'V' => generated.push_str(font.v.get(_index).unwrap()),
                    'w' | 'W' => generated.push_str(font.w.get(_index).unwrap()),
                    'x' | 'X' => generated.push_str(font.x.get(_index).unwrap()),
                    'y' | 'Y' => generated.push_str(font.y.get(_index).unwrap()),
                    'z' | 'Z' => generated.push_str(font.z.get(_index).unwrap()),

                    '.'       => generated.push_str(font.dot.get(_index).unwrap()),
                    ','       => generated.push_str(font.comma.get(_index).unwrap()),
                    '!'       => generated.push_str(font.exclam.get(_index).unwrap()),

                    '1'       => generated.push_str(font.one.get(_index).unwrap()),
                    '2'       => generated.push_str(font.two.get(_index).unwrap()),
                    '3'       => generated.push_str(font.three.get(_index).unwrap()),
                    '4'       => generated.push_str(font.four.get(_index).unwrap()),
                    '5'       => generated.push_str(font.five.get(_index).unwrap()),
                    '6'       => generated.push_str(font.six.get(_index).unwrap()),
                    '7'       => generated.push_str(font.seven.get(_index).unwrap()),
                    '8'       => generated.push_str(font.eight.get(_index).unwrap()),
                    '9'       => generated.push_str(font.nine.get(_index).unwrap()),
                    '0'       => generated.push_str(font.zero.get(_index).unwrap()),
                    _         => {}
                }

                generated.push(' ');
            }

            generated.push('\n');

            _index += 1;
        }

        generated
    }

    // Default font from @ferhatgec
    pub fn init_default() -> FFont<'static> {
        FFont {
            a :  vec![
                "   ***  ",
			    "  **  * ",
			    " *******",
			    " **   **",
			    " **   **"
            ],

            b : vec![
                " ***** ",
			    " **   *",
			    " ***** ",
			    " **   *",
			    " ***** "
            ],

            c: vec![
                "  *****",
			    " **    ",
			    " **    ",
			    " **    ",
			    "  *****"
            ],

            d: vec![
                " ***** ",
			    " **   *",
			    " **   *",
			    " **   *",
			    " ***** "
            ],

            e: vec![
                "******",
                "**    ",
                "******",
                "**    ",
                "******"
            ],

            f: vec![
                " ******",
			    " **    ",
 			    " ******",
			    " **    ",
			    " **    "
            ],

            g: vec![
                " *******",
			    " **     ",
			    " **  ***",
			    " **   **",
			    " *******"
            ],

            h: vec![
                " **    *",
			    " **    *",
			    " *******",
			    " **    *",
			    " **    *"
            ],

            i: vec![
                " ** ",
			    " ** ",
			    " ** ",
			    " ** ",
			    " ** "
            ],

            j: vec![
                "  ** ",
			    "  ** ",
			    "  ** ",
			    "  ** ",
			    "***  "
            ],

            k: vec![
                " **   **",
			    " **  ** ",
			    " ****   ",
		 	    " **  ** ",
			    " **   **"
            ],

            l: vec![
                " **    ",
			    " **    ",
			    " **    ",
			    " **    ",
			    " ******"
            ],

            m: vec![
                " **     **",
			    " ** *** **",
			    " **  *  **",
			    " **     **",
			    " **     **"
            ],

            n: vec![
                " ***   **",
			    " ****  **",
			    " **  * **",
			    " **   ***",
			    " **    **"
            ],

            o: vec![
                " ********",
			    " **    **",
			    " **    **",
			    " **    **",
			    " ********"
            ],

            p: vec![
                " *******",
		        " **    *",
		        " *******",
		        " **     ",
			    " **     "
            ],

            q: vec![
                " **********",
			    " *  **    *",
			    " *   **   *",
			    " ***  ** **",
			    "       **  "
            ],


            r: vec![
                " ******* ",
			    " **    **",
			    " ******* ",
			    " **  *** ",
			    " **   ***"
            ],

            s: vec![
                "  ******* ",
			    " **       ",
			    "  ******* ",
			    "        **",
			    "  ******* "
            ],

            t: vec![
                " ********",
			    "    **   ",
			    "    **   ",
			    "    **   ",
			    "    **   "
            ],

            u: vec![
                " **    **",
			    " **    **",
			    " **    **",
			    " **    **",
			    "  ****** "
            ],

            v: vec![
                " **       **",
			    "  **     ** ",
			    "   **   **  ",
			    "    ** **   ",
			    "     **	  "
            ],

            w: vec![
                " **      **",
			    " **  **  **",
			    " **  **  **",
			    " **  **  **",
			    " **********"
            ],

            x: vec![
                " **   **",
			    "  ** ** ",
			    "   ***  ",
			    "  ** ** ",
			    " **   **"
            ],

            y: vec![
                " **    **",
			    "  **  ** ",
			    "    **   ",
			    "    **   ",
			    "    **   "
            ],

            z: vec![
                " *******",
		        "    **  ",
			    "   **   ",
			    "  **    ",
			    " *******"
            ],

            dot: vec![
                "\t",
			    "\t",
			    "\t",
			    "    ***\t",
			    "    ***\t"
            ],

            comma: vec![
                "\t",
                "\t",
                "\t",
                "     **\t",
                "    ***\t"
            ],

            exclam: vec![
                "   *** ",
                "   *** ",
                "   *** ",
                "       ",
                "   *** "
            ],

            one: vec![
                "  *****  ",
			    "    ***  ",
			    "    ***  ",
			    "    ***  ",
			    "  *******"
            ],

            two: vec![
                "  ******  ",
			    " **  ***  ",
			    "     ***  ",
			    "    ***   ",
			    "  ******* "
            ],

            three: vec![
                " ******  ",
                "      ** ",
                "   ***** ",
                "      ** ",
                " ******  "
            ],

            four: vec![
                "    *** ",
			    "   * ** ",
				"  *  ** ",
				" ****** ",
				"     ** "
            ],


            five: vec![
                " ****** ",
				" *      ",
			    " ****** ",
				"      * ",
				" ****** "
            ],

            six: vec![
                " ****** ",
			    " *      ",
			    " ****** ",
			    " *    * ",
			    " ****** "
            ],

            seven: vec![
                " ******* ",
                " ******  ",
                "    **   ",
                "   **    ",
                "  **     "
            ],

            eight: vec![
                " ****** ",
                " *    * ",
                " ****** ",
                " *    * ",
                " ****** "
            ],

            nine: vec![
                "  ***** ",
				" *    * ",
				"  ***** ",
				"      * ",
				" ****** "
            ],

            zero: vec![
                "  *****  ",
				" *     * ",
				" *     * ",
				" *     * ",
				"  *****  "
            ]
        }
    }

    pub fn init_ansi_regular() -> FFont<'static> {
        FFont {
            a :  vec![
                " █████  ",
			    "██   ██ ",
			    "███████ ",
			    "██   ██ ",
			    "██   ██ "
            ],

            b : vec![
                "██████  ",
			    "██   ██ ",
			    "██████  ",
			    "██   ██ ",
			    "██████  "
            ],

            c: vec![
                " ██████ ",
			    "██      ",
			    "██      ",
			    "██      ",
			    " ██████ "
            ],

            d: vec![
                "██████  ",
		        "██   ██ ",
			    "██   ██ ",
			    "██   ██ ",
			    "██████  "
            ],

            e: vec![
                "███████ ",
			    "██      ",
			    "█████   ",
			    "██      ",
			    "███████ "
            ],

            f: vec![
                "███████ ",
			    "██      ",
			    "█████   ",
			    "██      ",
			    "██      "
            ],

            g: vec![
                " ██████  ",
			    "██       ",
		   	    "██   ███ ",
			    "██    ██ ",
			    " ██████  "
            ],

            h: vec![
                "██   ██ ",
			    "██   ██ ",
			    "███████ ",
			    "██   ██ ",
			    "██   ██ "
            ],

            i: vec![
                "██ ",
			    "██ ",
			    "██ ",
			    "██ ",
			    "██ "
            ],

            j: vec![
                "     ██ ",
			    "     ██ ",
			    "     ██ ",
			    "██   ██ ",
			    " █████  "
            ],

            k: vec![
                "██   ██ ",
			    "██  ██  ",
			    "█████   ",
			    "██  ██  ",
			    "██   ██ "
            ],

            l: vec![
                "██      ",
			    "██      ",
			    "██      ",
			    "██      ",
			    "███████ "
            ],

            m: vec![
                "███    ███ ",
			    "████  ████ ",
			    "██ ████ ██ ",
			    "██  ██  ██ ",
			    "██      ██ "
            ],

            n: vec![
                "███    ██ ",
			    "████   ██ ",
			    "██ ██  ██ ",
			    "██  ██ ██ ",
			    "██   ████ "
            ],

            o: vec![
                " ██████  ",
			    "██    ██ ",
			    "██    ██ ",
			    "██    ██ ",
			    " ██████  "
            ],

            p: vec![
                "██████  ",
			    "██   ██ ",
			    "██████  ",
			    "██      ",
			    "██      "
            ],

            q: vec![
                " ██████  ",
			    "██    ██ ",
			    "██    ██ ",
			    "██ ▄▄ ██ ",
			    " ██████  ",
		   	    "    ▀▀   "
            ],


            r: vec![
                "██████  ",
			    "██   ██ ",
			    "██████  ",
			    "██   ██ ",
			    "██   ██ "
            ],

            s: vec![
                "███████ ",
			    "██      ",
			    "███████ ",
			    "     ██ ",
			    "███████ "
            ],

            t: vec![
                "████████ ",
			    "   ██    ",
			    "   ██    ",
			    "   ██    ",
			    "   ██    "
            ],

            u: vec![
                "██    ██ ",
			    "██    ██ ",
			    "██    ██ ",
			    "██    ██ ",
			    " ██████  "
            ],

            v: vec![
                "██    ██ ",
			    "██    ██ ",
			    "██    ██ ",
			    " ██  ██  ",
			    "  ████   "
            ],

            w: vec![
                "██     ██ ",
			    "██     ██ ",
			    "██  █  ██ ",
			    "██ ███ ██ ",
			    " ███ ███  "
            ],

            x: vec![
                "██   ██ ",
			    " ██ ██  ",
			    "  ███   ",
			    " ██ ██  ",
			    "██   ██ "
            ],

            y: vec![
                "██    ██ ",
			    " ██  ██  ",
			    "  ████   ",
			    "   ██    ",
			    "   ██    "
            ],

            z: vec![
                "███████ ",
			    "   ███  ",
			    "  ███   ",
			    " ███    ",
			    "███████ "
            ],

            dot: vec![
                "   ",
			    "   ",
			    "   ",
			    "   ",
			    "██ "
            ],

            comma: vec![
                "   ",
                "   ",
                "   ",
                "   ",
                "▄█ "
            ],

            exclam: vec![
                "██ ",
                "██ ",
                "██ ",
                "   ",
                "██ "
            ],

            one: vec![
                " ██ ",
			    "███ ",
			    " ██ ",
			    " ██ ",
			    " ██ "
            ],

            two: vec![
                "██████  ",
			    "     ██ ",
			    " █████  ",
			    "██      ",
			    "███████ "
            ],

            three: vec![
                "██████  ",
                "     ██ ",
                " █████  ",
                "     ██ ",
                "██████  "
            ],

            four: vec![
                "██   ██ ",
				"██   ██ ",
				"███████ ",
				"     ██ ",
				"     ██ "
            ],


            five: vec![
                "███████ ",
				"██      ",
				"███████ ",
				"     ██ ",
				"███████ "
            ],

            six: vec![
                " ██████  ",
			    "██       ",
			    "███████  ",
			    "██    ██ ",
			    " ██████  "
            ],

            seven: vec![
                "███████ ",
                "     ██ ",
                "    ██  ",
                "   ██   ",
                "   ██   "
            ],

            eight: vec![
                " █████  ",
                "██   ██ ",
                " █████  ",
                "██   ██ ",
                " █████  "
            ],

            nine: vec![
                " █████  ",
				"██   ██ ",
				" ██████ ",
				"     ██ ",
				" █████  "
            ],

            zero: vec![
                " ██████  ",
				"██  ████ ",
				"██ ██ ██ ",
				"████  ██ ",
				" ██████  "
            ]
        }
    }
}