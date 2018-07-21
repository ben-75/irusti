use std::time::{SystemTime, UNIX_EPOCH};
use serde_json;
use APP_NAME;
use VERSION;

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInfo {
    appName :String,
    appVersion :String,
    latestMilestone :String,
    latestMilestoneIndex :u32,
    latestSolidSubtangleMilestone :String,
    latestSolidSubtangleMilestoneIndex :u32,
    milestoneStartIndex :u32,
    neighbors :u32,
    packetsQueueSize :u32,
    time :u64,
    tips :u32,
    transactionsToRequest :u32,
}

//private int jreAvailableProcessors;
//private long jreFreeMemory;
//private String jreVersion;
//
//private long jreMaxMemory;
//private long jreTotalMemory;

impl NodeInfo {

    pub fn new() -> NodeInfo {

        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let in_ms = since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000;

        NodeInfo{
            appName: APP_NAME.to_string(),
            appVersion: VERSION.to_string(),
            latestMilestone: "9999999999999999999999999999".to_string(),
            latestMilestoneIndex: 0,
            latestSolidSubtangleMilestone: "999999999999999999".to_string(),
            latestSolidSubtangleMilestoneIndex: 0,
            milestoneStartIndex: 0,
            neighbors: 0,
            packetsQueueSize: 0,
            time: in_ms,
            tips: 0,
            transactionsToRequest: 0
        }
    }
}