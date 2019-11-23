// typedef bool (*send_data_t)(void *handle, uint8_t type, const void *data, size_t len);
// typedef bool (*receive_record_t)(void *handle, uint8_t type, const void *data, uint16_t len);

type SendDataHandler = Fn() -> bool;
type ReceiveRecordHandler = Fn() -> bool;


pub struct Sptps {
    initiator: bool,
    datagram: bool,
    state: i64,

    inbuf: Vec<u8>,
    buflen: u64,
    reclen: u16,

    instate: bool,
    incipher: ChachaPoly1305Ctx,
    inseqno: u32,
    received: u32,
    //replaywin: usize,
    farfuture: usize,
    late: Vec<u8>,

    outstate: bool,
    outcipher: ChachaPoly1305Ctx,
    outseqno: u32,

    mykey: Ecdsa,
    hiskey: Ecdsa,
    ecdh: Ecdh,

    mykex: String,
    hiskex: String,
    key: String,
    label: String,
    //labellen: usize,

    // in c: void *
    // in rust: should be a function handler
    handle: usize,
    send_data: SendData,
    receive_record: ReceiveRecord
}


const SPTPS_VERSION = 0;

const SPTPS_HANDSHAKE = 128;
const SPTPS_ALERT = 129;
const SPTPS_CLOSE = 130;

const SPTPS_KEX = 1;
const SPTPS_SECONDARY_KEX = 2;
const SPTPS_SIG = 3;
const SPTPS_ACK = 4;

const SPTPS_REPLAYWIN = 16;

const ECDH_SIZE = 32;


impl Sptps {
    pub fn new(
        handle: usize,
        initiator: bool,
        datagram: bool,
        mykey: Ecdsa,
        hiskey: Ecdsa,
        label: String,
        labellen: usize,
        send_data: SendData,
        receive_record: ReceiveRecord) -> Self {

        let mut new_sptps = Sptps {
            handle,
            initiator,
            datagram,
            mykey,
            hiskey,
            //replaywin: SPTPS_REPLAYWIN,   // doesn't need it in rust
            late: Vec::with_capacity(SPTPS_REPLAYWIN),
            label,
            send_data,
            receive_record,
            state: SPTPS_KEX,
            inbuf: Vec::new(),
            buflen: 0,

        };

        if !datagram {
            new_sptps.inbuf = Vec::with_capacity(7);
            new_sptps.buflen = 0;
        }



    }

    pub fn sptps_start(
        self,
        handle: usize,
        initiator: bool,
        datagram: bool,
        mykey: Ecdsa,
        hiskey: Ecdsa,
        label: String,
        labellen: usize,
        send_data: SendData,
        receive_record: ReceiveRecord) -> bool {

        //let n_sptps = Sptps::new()
        let mut new_sptps = Sptps {
            handle,
            initiator,
            datagram,
            mykey,
            hiskey,
            //replaywin: SPTPS_REPLAYWIN,   // doesn't need it in rust
            late: Vec::with_capacity(SPTPS_REPLAYWIN),
            label,
            send_data,
            receive_record,
            state: SPTPS_KEX,
            inbuf: Vec::new(),
            buflen: 0,

        };

        if !datagram {
            new_sptps.inbuf = Vec::with_capacity(7);
            new_sptps.buflen = 0;
        }

        /// Send a Key EXchange record, containing a random nonce and an ECDHE public key.
        /// send_kex(sptps_t *s) {

        // create mykex, and ecdh




    }




}
