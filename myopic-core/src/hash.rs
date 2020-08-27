use crate::castlezone::CastleZone;
use crate::castlezone::CastleZoneSet;
use crate::Square;
use crate::Side;
use crate::pieces::Piece;
use rand::prelude::*;
use rand_pcg::Mcg128Xsl64;

// Total number of hashing features
const N_FEATURES: usize = 64 * 12 + 8 + 4 + 1;

/// Get the hash of the given piece sat on the given square
pub fn piece(piece: Piece, square: Square) -> u64 {
    FEATURES[(piece as usize) * 64 + (square as usize)]
}

/// Get the hash of the given side to move
pub fn side(side: Side) -> u64 {
    match side {
        Side::Black => FEATURES[N_FEATURES - 1],
        Side::White => 0,
    }
}

/// Get the hash of enpassant on the file of the given square
pub fn enpassant(square: Square) -> u64 {
    FEATURES[N_FEATURES - 6 - square.file_index()]
}

/// Get the hash of the given castling zone
pub fn zone(zone: CastleZone) -> u64 {
    FEATURES[N_FEATURES - 2 - zone as usize]
}

/// Get the combined hash of the given set of castling zones
pub fn zones(zones: CastleZoneSet) -> u64 {
    zones.iter().fold(0u64, |l, r| l ^ zone(r))
}

pub fn gen_features(seed: u64) -> Vec<u64> {
    return gen_unique(seed, N_FEATURES)
}

/// O(n^2) complexity but hey ho.
fn gen_unique(seed: u64, count: usize) -> Vec<u64> {
    let mut prng = Mcg128Xsl64::seed_from_u64(seed);
    let mut dest: Vec<u64> = Vec::with_capacity(count);
    while dest.len() < count {
        let attempt = prng.gen();
        if !dest.contains(&attempt) {
            dest.push(attempt);
        }
    }
    dest
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Square;

    #[test]
    fn test_uniqueness() {
        let mut dest: Vec<u64> = Vec::new();
        // add piece-square features
        for p in Piece::iter() {
            for square in Square::iter() {
                unique_add(&mut dest, piece(p, square));
            }
        }
        for z in CastleZone::iter() {
            unique_add(&mut dest, zone(z));
        }
        for square in Square::iter().take(8) {
            unique_add(&mut dest, enpassant(square));
        }
        unique_add(&mut dest, side(Side::Black));
    }

    fn unique_add(dest: &mut Vec<u64>, next: u64) {
        assert!(!dest.contains(&next));
        dest.push(next);
    }
}

const FEATURES: [u64; N_FEATURES] = [
    16981440902837726800,
    13389170232993973050,
    5915283093704561019,
    7787432441127208769,
    1966729422648614027,
    1004244818048038580,
    1853522389119074431,
    5287739706683080441,
    3538965732359089149,
    11236549938769817864,
    11273752147298579980,
    15166966677929695261,
    17897269199954134925,
    7383615875585469227,
    11675488791015230966,
    17705071937219203604,
    13910087315951721474,
    17672973962292435677,
    2087198845650289340,
    2489416606013789919,
    7686955509004980489,
    4111679370899898898,
    756142749130300811,
    7850773384198433747,
    7763590541416864709,
    12859767900442091124,
    9803371559138797389,
    1697588071027889763,
    11778885532974868226,
    5018691072048477544,
    17837264342147531266,
    17308470473709875841,
    3284999138407155866,
    2234540566939425335,
    11672597389975731742,
    113925859510141131,
    17888960742101554848,
    198522506003701973,
    14744030592063693996,
    9019334672976306733,
    15324628443818436974,
    4276739153523772669,
    14540712398942049040,
    181063307457451611,
    5267203788622667407,
    1396089518310541523,
    16443974734258105599,
    9012239377239449084,
    3767844762083120871,
    9043331136045802917,
    844898957469977630,
    10009333424106395143,
    14393513436980990656,
    6284950616819923260,
    17073520956005657405,
    8445934885230272836,
    1111260418580946510,
    7712373379082987899,
    257156913768387148,
    11493912022878268011,
    12457444285176387138,
    6032339089540612206,
    2097894935176106272,
    17729415393095682895,
    17534032059655281165,
    4939560762123033905,
    1000141795268252831,
    5820632148997907684,
    14801036260355049086,
    6347524598714327970,
    2257931013154887960,
    17574136123557516314,
    12782382684396665890,
    1417715927437219295,
    6139902329343993877,
    14642288864783344345,
    4238361023316997914,
    16584311436457832387,
    6494556909165396433,
    2188756563560302411,
    12298305304634443896,
    1245943159118438547,
    15683337702701008192,
    4637599955618130653,
    2023171518915183931,
    11664495601488365200,
    8431729122260036659,
    3583359942811457743,
    15644818981640393762,
    15797980515029460716,
    15832882640073614794,
    5459220189126676999,
    17819930588704181483,
    868080937993296704,
    15161010368006826719,
    17787188702677304430,
    8480075803561676901,
    1746601858436370175,
    16802781714871002273,
    44653435730856164,
    5676499869122172900,
    7212165138833167423,
    4308696437022378609,
    812032021344457253,
    1173849770843648003,
    9848460590623375532,
    2322319528786935699,
    1594862157284633887,
    3988132221382468736,
    1007525035915700746,
    11414912469915481540,
    10186086404849712877,
    11795848339474413511,
    16223744791485962524,
    2415104587434659178,
    15215287237009658751,
    12439538854653880460,
    8604687927379866499,
    12790413574688326185,
    10053548339088493587,
    16541227698935406507,
    12846642872508585709,
    14621297931877575620,
    8458091976445179009,
    765304077438221492,
    11384059072213645067,
    12807272510369817057,
    11748000481569239800,
    10756966683836440797,
    15959696851678323046,
    17852617901254585173,
    14051254448123051657,
    4249650985615670810,
    16771838534810592806,
    8793649601145643866,
    15557594545164035088,
    1367861461658931867,
    10173930793515036393,
    17706382436367035615,
    11386096032622286468,
    11822935571961106337,
    2822729993215684382,
    13622937390892810336,
    5974800057675400385,
    18005888695189415542,
    8982834618576565692,
    11992148366063726017,
    6212011146778419324,
    3383659900489864285,
    6638835942620756084,
    11082119825974763804,
    17475316062582275603,
    13865744833939933354,
    15877529096486741721,
    17140683834409562400,
    8533399242042307646,
    3533006719834017026,
    2728005726930704502,
    2239221915099941252,
    17766334833411449638,
    17071027450822358914,
    8275890668909283483,
    14207956512643963982,
    16592256798277286327,
    13828395856910416999,
    13967082832784369445,
    14931749123288390139,
    16555163081623900508,
    633176993048996814,
    14503502129152036179,
    11767639506575875563,
    1994212030061231329,
    17021907713973271310,
    4893568497044070130,
    14638442714878473590,
    14636665116956964595,
    12390700674343150133,
    1413389201753238034,
    4191496913777683463,
    10333086363940913677,
    4326026427228585078,
    2766616212145182587,
    12570475070181553637,
    7914420175756489549,
    14760771439434162185,
    3747514533968092237,
    10052882731959713560,
    17066018503833474582,
    10357311922340294723,
    15020520049297896670,
    14253796184179996730,
    10075417326645185200,
    12041896712706243156,
    804518457089768480,
    15437486209641432923,
    7425987104494752126,
    8994396476895312524,
    18413697504468812571,
    15925351811426033236,
    5684227400682757094,
    1689107101566877098,
    3771363221659470313,
    14400751621040591778,
    3341609887760577728,
    18237875477891244098,
    10639433611117548832,
    10787798666344498933,
    13831215910667312367,
    17608015098865130012,
    132452958223944949,
    4111767292333111578,
    16984194959545162343,
    3351599281569476502,
    2846756746018523073,
    8524174304920853646,
    10603763668260269943,
    15755100370626276626,
    4487446259928583508,
    5459264997129738112,
    1744723028406162594,
    7875016526527891974,
    5979590250000127130,
    15854105676621605923,
    17148162633319372221,
    15821897974454048866,
    7598445937254487125,
    4016504085236401372,
    11032095813687248072,
    9075175924292050135,
    3946391828261057609,
    9643001230593938973,
    2313592739345408218,
    6429686437017532404,
    4428083145575052647,
    352929674481287274,
    17598505218568505448,
    12401178125762131673,
    18225897226333848087,
    17914117453949797823,
    145554855894033257,
    3964662490342047409,
    1028393360377817957,
    14619461174996860866,
    5816148956936332410,
    10766704777215021180,
    7615441876473997751,
    9845001476110530488,
    7841919117518124809,
    5157253787288385022,
    11392908418860405534,
    17103499311285971506,
    1438916672526641798,
    16380451730072627543,
    3505636861781892763,
    15600834920739268830,
    13165042031134684012,
    15608370233919131127,
    3239456627947045306,
    12941161434483936349,
    10500069176481567753,
    12893424581692053839,
    7188782994947478447,
    15496250116470653815,
    7847491536997101472,
    7936129584880266747,
    3710199633319267639,
    2626049985507079870,
    10424956325123329890,
    17200125073067836748,
    16954668415805241211,
    17455667210555599338,
    16092005756880451680,
    5541155984531079993,
    1336926179890123504,
    13379839300735774,
    9608371535920101118,
    3659866219551783716,
    17170060472922074912,
    17704903928195410462,
    16983670829028408848,
    10469599759830762497,
    13308123700236060681,
    7744389734950958920,
    7199752856684522236,
    3792841997567633541,
    206064465786166147,
    5496195613434532627,
    8256085973168829683,
    7300968381888301207,
    2716280711465438845,
    8738088861610832956,
    5123460718434707792,
    4536554237769334173,
    7393911606551506096,
    3179025506913827578,
    4133277924410948673,
    2952112554688537879,
    5317453954694695626,
    5576063569527385142,
    10550768835483796768,
    402295702004574222,
    8142189163914186526,
    9775487735260993959,
    7226952715866944488,
    8663048547551794707,
    6122531019625024823,
    16473732477974489834,
    15374256359314784592,
    5939383355574455605,
    5764159518396661873,
    6964133938222096648,
    1251859055792163323,
    2480394503304691045,
    11111770319389450259,
    6988416286368443614,
    13090382075707415528,
    15326974929603839161,
    5825930622975554849,
    16053865982897240815,
    15625401903215454814,
    8868982248458153554,
    733182225232026238,
    12447733072768680678,
    14999337050394390544,
    7435880958478013268,
    3165014568029972803,
    1008549413798521684,
    11289180268186953751,
    18205912641350485375,
    4166607612991957421,
    9577694872297656799,
    3643867553648398102,
    11758086969277933213,
    10074550763735226627,
    5532255560108887464,
    10143184825563659757,
    11037812124482263105,
    4677210809614065621,
    7153464112453398022,
    12849688607795275308,
    18376736164052500860,
    10106641667055913233,
    4162210995984583537,
    7580027449532574891,
    8829529741413948026,
    18340447326015986115,
    11052127354184335684,
    12887247908437046838,
    17462561313857163641,
    14769723646935312044,
    10650848470902165013,
    4721498067522814014,
    3143346908491014487,
    9588370006728357240,
    15690566490456726334,
    16635900043156689319,
    8176429884922703958,
    8166070692866995446,
    17042979797157562074,
    8510065112081165238,
    7311025307581223246,
    3633907091117472612,
    17890362641335463703,
    10916278191791267908,
    9776460115793702718,
    7255522633014737394,
    10301770621111975548,
    14609416817730991232,
    5381155528052775982,
    15659803118380268929,
    10490995275500090707,
    8752193074546730450,
    13821959181540632351,
    3636963056329802195,
    8134784660434725944,
    18371911915287810509,
    5956986179371079789,
    8349774583723449314,
    8844149935911429816,
    10254407496782914897,
    781322695494250418,
    7267317324339976618,
    14507946596436328226,
    7718143025107680873,
    17014439746559604282,
    1411575457665869053,
    4465800980151424468,
    11500948080223210955,
    1469229226738898453,
    11779250622065611825,
    2712629813857438444,
    5773459174556854610,
    6446031086891076108,
    2508599485732312871,
    13373157478225896825,
    17757536621630891145,
    6996250219173292487,
    9156251719315275068,
    2075416526060790501,
    14436586815449702145,
    10873040221387067648,
    2524932959498357530,
    6863574893968730098,
    11374992319802592313,
    5895801599298340189,
    7421933909847235676,
    962511150887857556,
    9533234979899257054,
    5329544982663113766,
    9620336618469799741,
    6338146860962751625,
    1000593042181945210,
    6197789421261403045,
    2707611089238098254,
    2975823852531059811,
    7995018343840914623,
    11970866236657165163,
    18387312502732838610,
    2308360573047509169,
    11665504120051658400,
    7130978558192522447,
    6464444222806853424,
    4965731383174217691,
    13797827297061691568,
    11116112146854726013,
    13288348174511126767,
    17093003921995046340,
    6749813857991395702,
    17413004005182998580,
    13865618957165211596,
    8208466226224067928,
    3747271158820060877,
    7771852898908055027,
    6778966741854835118,
    14202461753638463482,
    10298186073163174324,
    10707842758271974847,
    7702810787605761188,
    10984387047855367675,
    7738276509738687748,
    9132415030329522204,
    13215904454678184973,
    17325106198738524878,
    14020473208318059251,
    13134332735787631373,
    25182375855953351,
    14702967354293303051,
    5814696463067909139,
    16994977048215955574,
    8377321384431296805,
    8288347687876361567,
    2546214541320911855,
    15423075881906584957,
    13364713454058539575,
    7001033024468371133,
    1311763047076445814,
    14104619009551497627,
    1661838026083404425,
    16017076139764770137,
    16982029739553298654,
    13749256774482774276,
    4135413926024345955,
    13660047004567450040,
    707149457515953623,
    13127756638687238514,
    8495448986762010217,
    6761225154573654751,
    10702034216256355369,
    15203893890477956506,
    1793442075017174216,
    10711593423996834704,
    9392214502028785226,
    11578502447332157370,
    6134288153781194747,
    14701381427442711314,
    3492750704896025621,
    5436470616468563672,
    18035327306298966057,
    11991941050772401773,
    15600996815899208992,
    7539279813326845556,
    15158352183505766349,
    11560319025986333054,
    1463076978199081969,
    5449620489226180089,
    2192877756289108834,
    240124960483381520,
    9791728250138369880,
    502507578166718858,
    6871925502662934902,
    16123766733030627545,
    17178142140031651002,
    8046301962344978312,
    13164531567077682765,
    4927705194122595074,
    1486517102808013844,
    3447971452897557972,
    7063010880855891412,
    3646171838134343398,
    15646119743012451433,
    15357475108019973877,
    15762553019167447197,
    4030191000102724403,
    14893045223380180099,
    2672484731029480206,
    2588453762148315381,
    15392604684421721176,
    12784856215211866098,
    1739738534772637274,
    10122978903649259448,
    10808397433983521478,
    841145325713238936,
    2363330484626913556,
    1158723944246010482,
    12372401037964017343,
    6562240992354539409,
    5763682497681598182,
    3132503880921237762,
    12370247871750289532,
    4576132994610759469,
    15055922860338762171,
    3227993278069578391,
    922776869874499472,
    14009471597853386235,
    15444883717274977867,
    17632831752328733257,
    2881177181469471628,
    18383210560349262569,
    12309470273209872052,
    16211561403310225290,
    14626287815607860901,
    451054182091668913,
    2829912774081243214,
    489587204535402571,
    11303109673277475490,
    6909741520975834973,
    9460965074755458941,
    5587561167253754566,
    2802071487948023739,
    16744047975830131922,
    16774931961449600258,
    17892345189125601744,
    16180261968508228381,
    2018869920186893441,
    2798503971911261182,
    9313949859643420667,
    160642059289901622,
    17492928090416439326,
    679582835213049563,
    12395217952366320637,
    9759931835646874119,
    15668468649046882066,
    12692621652486377521,
    10572640182039841571,
    2711875542311801281,
    3277199026858969713,
    7190694155822238562,
    15506548902919521105,
    11265585183640385044,
    14622014610653536633,
    10108940553519459942,
    6565802169047937594,
    1099186779805423199,
    10089378015720013894,
    12305241585212041896,
    13060196433876257048,
    3568009479028144685,
    12607844350062784463,
    12817285060733022316,
    15025805333104060350,
    6046181038975112643,
    14054068478134312330,
    4428916327995047952,
    2103371458587184597,
    15504633510571143549,
    16367172203648961139,
    9206314586700689782,
    18395057570397893463,
    12858484620459943457,
    7082804389892935929,
    14107959608625795600,
    15533075401077877362,
    805792539244624979,
    5911419190259846436,
    13607635742716850934,
    1156442824011560015,
    4461093234809374487,
    13412278306801653428,
    7638951972874474530,
    10461117339692397431,
    12999380474515218775,
    7126865034213407924,
    6874972581485957266,
    979034496620222987,
    9699822164670942569,
    12594135423526004314,
    17685639924926973677,
    17531738324265636704,
    1246528380808303450,
    10589626595403243353,
    4547057486255789694,
    716543655670283833,
    11961545459251072505,
    6130403706152456156,
    3212878284431368512,
    4668331299476369204,
    5771795439605986422,
    18299344349161333407,
    13029186050478255314,
    11143731813390608079,
    11433889198204399022,
    377515900033001821,
    5025861874165143653,
    11320487031268560458,
    11146003087953080393,
    8327224431235996258,
    5680947911510426098,
    8611732634055988347,
    17189504668331702972,
    2137704489870069337,
    1378448216505069965,
    8030709208362703452,
    8666613820958198890,
    13142842692868310384,
    6208160380629429843,
    18102431228369362346,
    13169648338726172716,
    11407300568792459755,
    18039663391910611745,
    18259747255051256104,
    4705998338316260846,
    1985091279822237734,
    16564609859225225918,
    10019551786908891335,
    216369030731738952,
    6151371219650668450,
    17535055610358758271,
    3352710064004427492,
    13227510456929008239,
    5922073173583441920,
    12298739905415478199,
    3852927506303466270,
    2147647967495813175,
    15263279347321183320,
    9157182273100737235,
    13885575554768108605,
    13573448658843315345,
    16824375270184292952,
    1946970976637008525,
    2288189900966236343,
    815548732335515572,
    1251259657162877680,
    2676432467796118281,
    984186782704629377,
    6243705798998213754,
    6673256566216044366,
    7525477542539551133,
    2153898177431574363,
    10789008581688134490,
    17128321264444936858,
    4041611272152290800,
    14423923597191096495,
    5191563333712839157,
    16449441985684390000,
    5742945053354308366,
    17578305991590899133,
    16063953382461797237,
    8893485931949406299,
    16928818683562099445,
    7937572989564352906,
    17773238151168026751,
    2271809131422587931,
    4290737518890822912,
    11402438845226454099,
    12956077576887103799,
    13064695591657350995,
    2185044125977727379,
    15390028727184137458,
    7801826940976619161,
    7851518661083293234,
    8091934838373487108,
    9727291491341875092,
    685853814980118501,
    2238489935872807491,
    8775731037240587065,
    15280174713831115332,
    12226689027174362608,
    8985738288679733702,
    2006652145239649626,
    420643544891119691,
    1535027598839898231,
    11997988936390993495,
    15960561717919773693,
    8597131750489177648,
    9624905229902601141,
    5398809140237935360,
    196857438571320955,
    9394527300968427926,
    9033177375321503119,
    7361795198702195134,
    5915250907338674296,
    2965396197842727184,
    7913536996188062647,
    5968459071968716419,
    8962580249121889293,
    3099943052943679477,
    12717497320350185143,
    8009978572527643229,
    3728055697131702034,
    7461325579688461571,
    6985286124887288900,
    13324503245133103659,
    177065616115990004,
    5646036415599138033,
    16497152413789738698,
    2388677939155355876,
    6636290434354156210,
    14803977265134405591,
    4007065349627048055,
    16735476974568699126,
    9875146173162828192,
    14992560426402473085,
    1268893318567863370,
    12249474062520691542,
    12460475188027312479,
    12579284938590547355,
    1205218504956649615,
    369734714373705010,
    13850332846202501051,
    14751777331135717738,
    11294452961942994790,
    10975434618846997101,
    2562844337082134647,
    12188079577320643577,
    3728153480303386379,
    12036820635161006969,
    14342648372940053851,
    17333249371434531263,
    12591994446587538518,
    15895638948292048463,
    10245061382320656067,
    6613861251595812103,
    7786024098247637678,
    8016291687622422464,
    11987553914083441411,
    14868395867193960624,
    1718162313549290514,
    15786837862207838031,
    6806487083107602322,
    18353218416970874198,
    1507521568313343916,
    3897906849464647275,
    8953454096290763151,
    3285831659581931643,
    5545324167674857003,
    10132103878284564531,
    17452872784917814398,
    3005326549973574238,
    1359058237790886262,
    11342424127833138443,
    88862214912896100,
    8149710741706528412,
    5338325291441386679,
    1908916719817830424,
    7731887820847009561,
    265127237898025079,
    13770838568219628902,
    6807950187556197467,
    3323165315149945038,
    6365648104236759924,
    1621102043423766732,
    10249336505490214576,
    9616021352835475313,
    9563975895673580657,
    15612122441449425634,
    13367508551891674842,
    10705688998729503783,
    10350016333854978882,
    5089885123724170365,
    9843661064086812320,
    5963312476232014387,
];