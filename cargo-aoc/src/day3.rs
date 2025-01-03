use std::iter::Iterator;
use regex::Regex;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> String {
    String::from(input)
}

#[aoc(day3, part1)]
fn part_1(input: &str) -> u32 {
    let re = regex::Regex::new(r"mul\(([\d]{1,3}),([\d]{1,3})\)").unwrap();

    input
        .lines()
        .map(|line| {
            let s: u32 = re
                .captures_iter(line)
                .map(|cap| {
                    let a = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let b = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
                    a * b as u32
                })
                .sum();
            s
        })
        .sum()
}

#[aoc(day3, part2)]
fn part_2(input: &str) -> u32 {
    let field_re = Regex::new(r"(mul\([\d]{1,3},[\d]{1,3}\)|don't\(\)|do\(\))").unwrap();
    let digit_re = Regex::new(r"mul\(([\d]{1,3}),([\d]{1,3})\)").unwrap();

    let mut enabled = true;
    let mut sum: u32 = 0;
    field_re.find_iter(input).map(|m|  m.as_str()).for_each(|m| {
        if m == "don't()" {
            enabled = false;
        } else if m == "do()" {
            enabled = true;
        } else {
            digit_re
                .captures_iter(m)
                .for_each(|cap| {
                    let a = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let b = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
                    if enabled {
                        sum += a * b
                    }
                });
        }
    });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let output = part_1(input);
        assert_eq!(output, 161);
    }

    #[test]
    fn test_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let output = part_2(input);

        assert_eq!(output, 48);
    }

    #[test]
    fn test_part_2_b() {
        let input = "?% mul(948,148)why() %how(670,744)mul(590,32);where())#}from()>how()mul(611,372)}{~^?>from()^mul(835,665)who()]#^don't()select()select())mul(724,851)[>&mul(188,482)$mul(781,111)[who()<why(),!]mul(678,13)why()$#%who()mul(620,771)<!^}@^+what()mul(281,719)(]'what()where()>&from():!mul(147,678)how(){mul(938,510)where()!$?*['mul(103,563)where())mul(4,125)$*>>^mul(126,929)]& %~mul(161,418)who()>>do()]-''?mul(416,366)~?/where()]who()mul(459,47))>what(){@[(mul(219,400)+do()when()from():who()when()]&{{%mul(804,830)-select()what()*what()%}mul(861,992)who()!',mul(159,874)#<)''<mul(460,777)?mul(909,244)how()+what()]<do()?}mul(749,87)from()(who();why()mul(430,124)/$>how()@$%mul(214,139)&how()>mul(112,835)select()*from()@why()?[{mul(209,568)/; ~)mul(630,749):mul(841,589)/;who()>[mul(778,567)+when() how()<#mul(544,851)what(){+mul(327,103)from()what()/[~-mul(995,415)/when()-mul(880,153)}:}mul(368,920)'how()mul(864,419)from()what()@mul(208,291)who()<?}?what()',[{mul(575,454)*&(<{how()[mul(557,489){{why(){how()@who()~mul(423,703)mul(910,916)+what()^/<-*from()'mul(746,826),-*)/+>}^from()mul(154,571)++:>,mul(601,458)why()<;how()~from(172,16)mul(333,315)?[mul(513,260) {*mul(117,759)%]mul(77,644){($%>]&~mul(238,306)~select()from();-'who()'mul(460,352); ?select()>[[(from() mul(337,294)why()how()</$<where()don't()(?]{why()%}from()mul(367,653)~mul(910,873)^why()>mul(499,785)>what()[*:#where()*what()mul(765,210)*$[]mul(461,957)##)+}when()-@:mul(198,90)what()what()how()') )mul(258,966)]+(when()mul(535,417)where()!don't()@mul(939,319)?mul(751,538))! mul(758,675)~how()[how(),@>[where()when(29,965)mul(358,39){^what();/(where()how()mul(271,786)why():mul(792,761)do()$]%mul(740,232)>who(949,378)what()[(where()who(){who()#mul(595,343)%+mul(194,296)'mul(161,747): '{where(12,567),@mul(234,39)!+do()/who()[where()&'what()when()how())mul(138,925)),#;where()>{mul(738,864){mul(605,662)*when()%when()+( /~&mul(633,935)when()];mul(263/}*<!where(),- ~when()mul(512,798)]}where())when()who()mul(933,447)where()}mul(33,935*mul(15,975)mul(574,550)+#^;'$from(280,157)$^what()mul(919,849)@mul(18,160))$&^]how()what() when()where()mul(88,657):/from())+:/when()@]mul(71,74)from()'*:@{>mul(127,821)^how()$$select()select()@^{:mul(867,979)&%/>{%^how()what(499,657)+do()%what()(~;-:*mul(438,941)<]?]mul(208,834when()&^;]from()when(613,710)^}+$mul(809,573)mul^)*:from(379,983)mul(47,786)}when()-what()how(450,632)> where()how()mul(810,597 ;;{%(select()select()&,mul(356,249)from()/!{#&^mul(23,248)(!who()]-+,mul(873,987)]{what()<  )-{^mul(591,317)/mul(382,188)mul(476,338)*why()$]mul(865,625)who()})?select():*@[)don't()/ ,mul(737,418)select(318,357);+ what()<mul(41,445)mul(236,630)$}from()]$^$,(do()-select()mul(369,197)from()]#};^mul(561,752)+&#+}?}:mul(18,235)<'& ,(*mul(645,811)why()select()who()[>where()don't()%#>!>/@what()[mul(490,823)&^( ,'@ [do()@mul(855,491)*^why()[,mul(348,679)how()$who() '&how(16,459)/!;mul(43,422)#^from()![}select()mul(976,749)-}select()-where()select()mul(223,589)%[why()mul(868,881)mul(178,790)$,{who()from()#,mul(318,399):where()?[mul(182,864)where() mul(156,690) -]mul(857,353)#'%,},>?+@mul(914,528)where()$mul(785,748)<$who()[mul(453,859)%'@ mul(84,729)/{do()(?$<}mul(820,286)?:*?}#when()(%mul(245,958when()?from(),+mul(128,335)mul(463,102);:]@-~-%mul(914,398)";

        let output = part_2(input);

        assert_eq!(output, 24842536);
    }
}
