// convert starttablets to plando

struct Randomizer {
    plando_defs: Vec<PlandoDef>,
    start_items: Vec<LogicKey>,
    progress_items: Vec<LogicKey>

}

const SMALL_KEY_MAP: () = map!({
   "SV Small Key": "Skyview", 
   "LMF Small Key": "Lanayru Mining Facility", 
   "AC Small Key": "Ancient Cistern", 
   "SSH Small Key": "Sandship", 
   "FS Small Key": "Fire Sanctuary", 
   "SK Small Key": "Sky Keep",
   "LanayruCaves Small Key": "Lanayru Caves",
});

const BK_MAP: () = map!({
   "SV BK": "Skyview", 
   "ET BK": "Earth Temple", 
   "LMF BK": "Lanayru Mining Facility", 
   "AC BK": "Ancient Cistern", 
   "SSH BK": "Sandship", 
   "FS BK": "Fire Sanctuary",
});

const MAP_MAP: () = map!({
   "SV Map": "Skyview", 
   "ET Map": "Earth Temple", 
   "LMF Map": "Lanayru Mining Facility", 
   "AC Map": "Ancient Cistern", 
   "SSH Map": "Sandship", 
   "FS Map": "Fire Sanctuary", 
   "SK Map": "Sky Keep",
});
