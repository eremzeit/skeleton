use constants::*;

pub const PIECE_HASH_COUNT:usize = 767;

pub static mut PIECE_HASHES:[u64;PIECE_HASH_COUNT] = [
    11222789421819506, 9153323150103610931, 16364798908152400623, 12205909498157858750, 16106927289660083879,
    13790507506110712980, 14392098937865322729, 13182242364510143186, 7770452950235945728, 6660371171771578723,
    12377466353845021944, 12825539298306953845, 284652366426581930, 11929578780752775533, 2070861031710443468,
    9027397836849756559, 3178249697200715095, 18161112155210982288, 15467332615342126065, 12622167316566732469,
    3350430361004257047, 17015216146186692051, 17608076959990004472, 7248113270008823377, 15704913271526112179,
    17352121559836812833, 5564777481643554629, 18339169492670948946, 5916393958444774906, 15071353331254931761,
    15410485148422358758, 3212078973404786877, 11205349672839566507, 7240607469122094486, 14470875700727068803,
    9427834402992434083, 12772015793468712861, 2412174785324071042, 9953040867230513151, 4844651129375137583,
    4389381293794815569, 7000771210705714497, 9175918232208573346, 2968160282805218091, 14610834771367257641,
    2305152519519427041, 6383530876873842935, 3247854576350993791, 6757928611686254354, 1706396452540066176,
    2705496412968621850, 578227614465348580, 12370785626998229177, 1740396737722150387, 14181934710668082805,
    17144450723299087880, 368787300118196140, 4882494890969636186, 15302513386444339209, 7179985533605657642,
    7756835479474716053, 526900734507119453, 14542050379862578546, 11246401422917025943, 16318460568693623152,
    9949773944573458250, 2003536770147437859, 571483614046815648, 14193124436478241345, 16940229050380304453,
    14037022656490045931, 16546761451461406398, 4315893891761025040, 18290953264184099538, 2745645722180936864,
    12317098907071743467, 3486056398483947503, 18212063926108975172, 12346175613811489163, 3082663498030254591,
    3257082162391837878, 7013816327910910724, 10869475594358354258, 7871297256431141025, 6281620496994604367,
    4182251959275524687, 5738643717043922463, 11983163159802247766, 8196417793880636255, 340346788428884034,
    1789748440510238370, 4226709180572443152, 9016547050293611120, 5489013313449378926, 9086194046328901300,
    8067084263230122489, 13516251246202486375, 3869541428141627291, 7342298793354779776, 16223132981464139851,
    12224629878364936256, 3369012308693996457, 17014120954462490983, 4133933006298777375, 16410743024320595274,
    2104000459810868417, 3376931985337121111, 5722736324883109105, 5711616641510203254, 16815850878735858770,
    8634342085027552550, 15242550854602592041, 5078157079745530745, 2782044630014620698, 11848101042655197776,
    15379379740784992654, 6131319308876997701, 8089272391075960539, 9003190859229027530, 12553330234542979490,
    2554157900057418379, 15831030586479225950, 11771459037146258178, 6546030409894263449, 16734022603910016699,
    6684660471341333643, 12284358330074949833, 16082893016393304944, 9102984100865779462, 9595989951521602045,
    7306410306432909664, 8886131976147029862, 14388634140664494819, 16307981541589789225, 14475143817226131736,
    5489607922301590309, 6348333267097558892, 15063948001234786306, 35554214723129126, 5351283403548322355,
    8346810862212494765, 562268108558364785, 5363439606397584932, 18375730486477728346, 7384914099284783389,
    6906728770863135093, 750011342437191973, 13157516929591734199, 10546388972802842977, 4888228780712980689,
    2316125862707508574, 9479520602370370342, 14249354573662582424, 8199219579439932015, 17371596732357716115,
    5492487370256266588, 3810477856298232828, 10136069823057305562, 6741141255983813437, 10465492463763849296,
    7231422771851771169, 12666864741540343837, 14872220219580945137, 2370595034121901912, 6962403073498502176,
    3796644169891387106, 3588871716387585246, 6581930032358812341, 12464496846387606382, 13292838871854290192,
    6810949766757990286, 12220974811249379289, 16843473257769307351, 14547224077674014080, 7209340140122649722,
    18054092764741154449, 10744948697591252114, 11711771236306074649, 5598254048654299655, 2224217765452147387,
    3498081547081492137, 1296223655596607529, 6680814262056937054, 11370248504780528988, 2297061009104106804,
    13568886083741784863, 10709079746847813284, 4584444677322387907, 3575625734347166488, 1030974986851180196,
    17766244375982732196, 16042229334556161041, 12669268560898960267, 11714702284871854258, 3366310881830910292,
    8767226521558924740, 5423568355738365874, 18158378420816497455, 4435116150029376246, 6357958722443254698,
    17895110025208965296, 4904584934363238388, 17222689622655455567, 15226190934875532670, 10558602772219567750,
    1710038611951929482, 18374849893183975592, 7543413771115018527, 4785414012424330818, 5387356203595601350,
    14689468497639319628, 16030937068612461600, 14163957674690248769, 10892929005057084038, 4264065693578134239,
    517636862360749602, 3854836039798023330, 6173623218485339293, 15064287990728831331, 11428407977077193370,
    13312714910147983222, 15429281236177071824, 1454253670159245425, 18055820170624710090, 16847867493842127454,
    13506840429427067149, 5043100623106819808, 11626982923775603341, 4954142576075221772, 17269353922988282088,
    9506569508741885932, 6319609264755024948, 10141367015544377257, 15288528440908484965, 4050203897025381797,
    7667045472524939361, 9822622609838427565, 12549301991464360589, 17366322766268708917, 7199157685530790294,
    12308686641273378538, 3109389747916998221, 2506261311533846183, 10420863978645615865, 17645389333207410346,
    7528943665546068366, 13811418428072246132, 8242060210726864663, 11121237725007546524, 14022609672262925323,
    2634605631264754379, 6044193491181677608, 8077275144627097188, 7732796123643014233, 16569843028870983912,
    13639864515158339053, 11060504405844441462, 923670525921443, 3251576258818715841, 1607650128510817813,
    12913102914125154741, 3887121639602902904, 9635090867585029864, 9352039637685424426, 1930753572499386774,
    1663017888490614324, 13971658415392526755, 8846496983723736972, 3347872980440237524, 11462694091048759916,
    637778603433476516, 17052951315357730303, 5618402019021287911, 530184518970737017, 12869798183748686403,
    10035592423722811975, 18259911803673053750, 6539958736530142154, 8005511266661130922, 3833652080546137006,
    17511124828868945334, 14729657966412262713, 7386392622144916857, 12386583727147634749, 11332008909192809476,
    6007505014650338416, 9432965182145577782, 2253308905933933201, 7107957695208026835, 4191647261936066358,
    14646370045043269032, 12161498350946810655, 8368767221779291034, 8226893622042653656, 6857176190834119513,
    15630507744965803661, 12047613022797953374, 5900013004794281437, 15575868251567103957, 15133337490660363166,
    1357284340115784327, 10678070804159695251, 168836256860967070, 15243442360263774456, 18353480205066188669,
    10550343143764299267, 2061438091071331092, 10444474839450372355, 8606365951794775302, 721311832893179368,
    8247373661423961075, 10184836814102248872, 3810828238648562582, 6508610962839730437, 9466065814768360589,
    5456899048993791691, 10955424334646945200, 6360341970094052921, 7878642028188452227, 9788007441266744450,
    5265582936769611206, 33702303955499775, 17945182129961610933, 2273662069927097457, 9532263518748648820,
    13412766690120971246, 16949604898641998401, 17833312548221723639, 13932236078623290219, 14561126430390152024,
    12015022496313261938, 3884830578914215080, 4125052438151538245, 16571647306234757434, 2710816979006560547,
    10562406278986540445, 11319256054640830952, 12246957011897916304, 901495610910041453, 2522682249093572592,
    10558241315360394072, 5788673262756737863, 13230437244819788035, 9070080611128487855, 12540904728088257432,
    14983726043255822371, 8875118939603500053, 8451415751314669615, 10152141388785529664, 16826181106187780784,
    17948240990082439988, 12571543820196591379, 4767380530394849415, 567692215971694592, 402756991216858629,
    14297090705413907856, 10871784890113845821, 3892530200075671137, 13211194160726735680, 1896638142139249483,
    7411857436421170504, 1605667928048660469, 6212083478833588234, 14209023938697551782, 10695718081766912628,
    17140352622858172825, 9868167360349254361, 16786981802464493355, 15670502319247300312, 14859170016671991774,
    10675556045654120713, 4198683653382490028, 12000113075196468723, 10148054997334921452, 9958304846462013060,
    17634239177599966388, 11995111209559263390, 10952323702248860423, 14870448647209242335, 10522629914436562932,
    13450804694559158287, 5824699313838330403, 2071261098272158668, 2849430601784402495, 14877315442229048581,
    16538805738400964705, 1714273536465106211, 13402239048809767224, 7377760845966933201, 4018140858198665679,
    18146085187870380570, 4425954152556630364, 17611668746926280420, 17106509864985367, 16640187959037259202,
    2392294952917694, 10707948285830752602, 16798330845579178181, 2800213065694137879, 5084792965140657904,
    190742193338071616, 1796481568863553204, 57469810206398928, 4321088995825612894, 9622182310563445447,
    12816439427363440527, 7751337721670086220, 3655868314397000737, 16820558599691339030, 2437443252766830474,
    8980521688085029953, 1741233833506317189, 10478736595915220003, 5541903550308735478, 6802870199582945834,
    13373472932202169306, 16958585606822156890, 4662115703275565126, 5980181370427533776, 15295411773481781139,
    6872638879964659365, 17534343361386792832, 6341571016391195989, 15693897211086419238, 8180553143337605170,
    16552025513933076059, 17800313549998747028, 2925927693507052588, 18040967464463271669, 8791415399504791414,
    489374532806908213, 4591682942051537784, 13141811216693099090, 9801184040519582265, 1559554977153008568,
    17022007002128915980, 10549225238337042757, 6539240546992065432, 3574680427561462152, 11639300652507281695,
    7732197082339799086, 15525438961766992783, 5617978552349910554, 15085142818208583657, 17289958335989751205,
    6303577217092021377, 10190790636495834709, 8218087214249206321, 15114917513855477187, 12072556626064530554,
    9531247670033735998, 5262404131149237441, 3744065287947932996, 12730059921447053650, 13378460576751363271,
    11416440853754524585, 14925348913388853687, 16122809904308975036, 14334588960196585335, 804583977916526946,
    7967017894388311289, 2813093715925775051, 17474455465201218402, 5058180208536412645, 991594436683539263,
    2874580234310321478, 14597333900578141691, 1426896931846935816, 15692157547723768974, 10118954640392159675,
    3915043246931481085, 12485460156865572992, 7627055003087738582, 1976640330778084329, 13537898883222962462,
    10707150160121079716, 618369529950295304, 11178945942835720397, 3364720174509349021, 849547486864467974,
    14058593936205045232, 10698847286946945472, 11864908362651495326, 18263795271095971035, 14762848546680460460,
    4210764949308600753, 11018507854865400107, 17787265392318558160, 12447022237331079756, 5402445558615997097,
    10212550529485698882, 15312635544381400340, 2783519533357381006, 15651630417872069095, 8879237745875135279,
    3129749620517017924, 7429756211626195385, 17796033720447016965, 16580784187234169551, 12865268566173577546,
    2855418125323464803, 15115372350303336267, 14245114439206586339, 8960891940382689870, 5982801661313309010,
    8055254781133784202, 11205438776384850055, 8056651959905535161, 10846856583480193833, 9784249831919155498,
    7265353275047299723, 17026936314348090408, 17688476557600334121, 12796784650547978782, 2565202695204699136,
    3619347412338082791, 5344881911890971456, 14550209343496886609, 11082956422718551953, 2577409223880732537,
    3731736790643754399, 7066956403286966890, 8456778718984836655, 8041844908467793830, 12546970054241337860,
    13222656594737988507, 5787923179604624179, 12015179435198738080, 11306285504105774089, 3845077846663616544,
    6731514969110164529, 3495340110564952032, 7209815863504002966, 9028485164717600059, 17795405702977397362,
    16411447997311255883, 14753576452671708387, 5731162790620333960, 12842213811054336337, 13798371240079998839,
    5020338699829602111, 7245115769291645541, 16341798420906879786, 18183835206168556196, 13575742875175120372,
    15791068587311091223, 3366985432191345933, 15440948124152731334, 16076997014763987847, 6833303372301057263,
    489366082496826380, 10029881897125928787, 16622336824741242036, 14710331424704952550, 14657738937412015612,
    16469300499023842003, 15599549234261321032, 6766402236957910250, 3988769390134272104, 7167035174062567191,
    3010244558006247309, 12820089725525737672, 13605040695548965676, 1751630792466158197, 9685462637761866584,
    3908079709676438396, 807933567753684021, 3927811359207248261, 14158478175321300109, 12388788237429395649,
    12914642816739577477, 16599930697580874626, 7472156460568144979, 15696590832147976771, 1341068796180918548,
    10230813624123332182, 15013566991569457990, 10730795873546819763, 18110210746486413268, 2419148048573763948,
    2749943285737023654, 11506096932370171912, 12397602398623654400, 4917479799831661850, 7655276437869613582,
    6457139086769803632, 16834363315015569451, 14235019627204885896, 17023828133488162596, 2110714883801936158,
    13286042646823751651, 5186349426651448504, 4889226890264838141, 16204929850523389918, 1376063016939629050,
    15838878879235640784, 6488511273135908746, 1460687928741216325, 2951320806442560095, 4218320312613433176,
    17876259074247539693, 2485402189445893297, 9244177222696022489, 14192262898426607864, 14535134113712015415,
    5799255075060246991, 4142293507656859447, 6171817436629482140, 15790329944164366776, 14474120636749033616,
    11442884380253032752, 3991646130497560707, 2403584546202615542, 7635786993701615647, 12934269599926903623,
    11400995302401825062, 9646901307049085273, 11218309675749836146, 12673421295054430673, 12013693290749522462,
    4868427536516952864, 9359662329236832655, 9506180873105256392, 2948984016729425430, 4415364819804242399,
    4327294921235472232, 16646854554305469410, 5081653856162679540, 10078275036706728748, 10301192439567828776,
    11954028733614973344, 17135116829044275982, 6491967966789159774, 16427046116476127625, 11533862895495761393,
    1855879290074141055, 14737632355142835770, 12593487132024825042, 3017582811280899361, 9594503185838915814,
    4154078357848944965, 9683384679881213713, 1286485250555139191, 4580195254891163811, 6267618193164545107,
    9660193531065862033, 13049523169909361862, 10032363717070757186, 14434732273980750346, 13338728441247003917,
    3845393489191120974, 9777682353000823159, 2940181410425173780, 16195198586294455207, 12829700606302127039,
    11021065118832105194, 3070349111936429073, 15070644116835278279, 5373507007253303776, 8107366089793830524,
    7287116606106051092, 3007460836091105073, 8391884728297963059, 11798216131715795678, 4289924105312729327,
    18004396082368443781, 12190675787979838250, 6920456421710775315, 11126585390707270016, 15170312510023715475,
    15540467051910461120, 17191027618573183484, 9313699156751663110, 18016758534117535545, 6205267041834092983,
    6262049715412142541, 1319416099924987340, 15014701556322103427, 7001795556913644724, 12748587525784968558,
    9483042112904106564, 596515427299082532, 17280077601416213849, 10755118786719572327, 7522251186185889616,
    15759833218919826336, 10896424306418186134, 7223039073149020391, 17613621878590884870, 7809641025236986938,
    10363213550708570259, 8593095527354624316, 5526754198732002195, 7912156484094415277, 10018711510607257416,
    7649881282982127297, 5791255171335072961, 17077383331126505946, 11571045720809348021, 8852230437821043279,
    15448926972321684495, 3654082939811171497, 7452388546951860592, 10492096748270100627, 12305888665699816901,
    7585698092520906516, 11245271191521904190, 15469591654417918447, 17509587166132375516, 13253774642960397725,
    14631936416409904907, 15229221009430494414, 11235515433666964570, 15165327624033968289, 1379962194913379013,
    3168705108933367309, 12065002591360852580, 15555931690675111152, 16951853939670285418, 4118305554958320424,
    8050237192985322035, 5943467963285594928, 11525138795090318024, 15728408078443078608, 10886675751018017485,
    15425149251416884026, 11863940887702355574, 7633567883294973309, 5092469458542882278, 9839581631048772721,
    3676345267831052495, 11665899322737401822, 5505748126636566418, 1628278964671276174, 5012858627494910092,
    4210134752745632455, 17920264005945387490, 14475845904176003959, 1940365572216036116, 6349831241557530380,
    7419584798370412022, 13224278238713245161, 6337256240266269106, 871789474410759009, 8709900106588603555,
    12067472165140888695, 9989887103323116131, 7027196502148871761, 6880208039470118050, 8323165489952960909,
    607350626500335214, 6666725240152766982, 1687053534489382806, 12328131339850853384, 11559982418575634106,
    17566585846146960384, 10922373794099033088, 11482684698157628367, 15304889586663014529, 17967907710355255375,
    16450747819677576509, 1554176043467167716, 6370737674100262542, 17580889322917475720, 18224892881340579985,
    14775902519097677696, 7143891375031270064,
];

pub const HASH_W_OO: u64 = 16740397257515075237;
pub const HASH_W_OOO: u64 = 10187754592047244723;
pub const HASH_B_OO: u64 = 4596077665422651169;
pub const HASH_B_OOO: u64 = 893064223118783232;

pub const HASH_BLACK_TO_MOVE: u64 = 7138248122279521201;

pub const HASH_EN_PASSANT_FILES: [u64; 8] = [
    6291394906689225757,
    15506210628919896947,
    15239218664328381020,
    16294577894806826787,
    3916530409267059420,
    13432217411389072158,
    6744670793002844358,
    8140250880317121223
];

//pub fn init_hash_array() {
//    unsafe {
//        if !IS_HASH_READY && false {
//            let mut rng = thread_rng(); 
//            for i in 0..PIECE_HASH_COUNT {
//                PIECE_HASHES[i] = rng.gen::<u64>();
//                println!("    {},", PIECE_HASHES[i]);            
//            }
//            IS_HASH_READY = true
//        }
//    }
//    //            RANDOM_GEN = Some(XorShiftRng::from_seed([1, 2, 3, 4]));
//}

pub fn get_board_hash(pieces: &PieceList, is_white_turn: u8, castling: u8, en_passant: u8) -> u64 {

    let mut h: u64 = 0;
    
    for piece in pieces {
        h = h ^ get_piece_hash(piece.0, piece.1, piece.2);
    }

    h = h ^ get_to_move_hash(is_white_turn); 
    h = h ^ get_castling_hash(castling); 
    h = h ^ get_en_passant_hash(en_passant); 

    h
}

pub fn get_piece_hash(piece_type: PieceType, file: File, rank: Rank) -> u64 {
    let square_ind: u8 = (rank * 8 + file) as u8; // out of 64
    let hash_ind:usize = ((square_ind as usize) * (PIECE_TYPE_COUNT as usize) + piece_type as usize) as usize;  
    
    let r: u64;
    unsafe {
        r = PIECE_HASHES[hash_ind]
    }

    r
}

pub fn get_to_move_hash(is_white_turn: u8) -> u64 {
    if is_white_turn != 0 {
        0
    } else {
        HASH_BLACK_TO_MOVE
    }
}

pub fn get_castling_hash(castling: u8) -> u64 {
    let mut h: u64 = 0;
    if castling & W_OOO != 0 {
        h = h ^ HASH_W_OOO;
    } 
    
    if castling & W_OO != 0 {
        h = h ^ HASH_W_OOO;
    } 
    
    if castling & B_OOO != 0 {
        h = h ^ HASH_B_OOO;
    } 
    
    if castling & B_OO != 0 {
        h = h ^ HASH_B_OO;
    } 
    
    h
}

pub fn get_en_passant_hash(en_passant: u8) -> u64 {
    if en_passant != NO_EN_PASSANT {
        HASH_EN_PASSANT_FILES[en_passant as usize]     
    } else {
        0
    }
}

mod tests {
    #[allow(unused_imports)]

    mod zobrist {
        #[allow(unused_imports)]
        use board::Board;
        use constants::*;
        use super::super::get_board_hash;

        #[test]        
        fn test_get_board_hash() {
            let pieces = Board::from_fen(START_FEN).get_pieces();
            let mut hash = get_board_hash(&pieces, WHITE, CASTLING_DEFAULT, NO_EN_PASSANT);
            assert_eq!(hash, 5295089526049533461);

            hash = get_board_hash(&pieces, BLACK, CASTLING_DEFAULT, NO_EN_PASSANT);
            assert_eq!(hash, 3056757801067152804);
            
            hash = get_board_hash(&pieces, WHITE, W_OOO | W_OO, NO_EN_PASSANT);
            assert_eq!(hash, 8851735708471636532);
        }
    }
}
