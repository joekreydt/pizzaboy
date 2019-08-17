# [ doc = "timed presentation related wl_surface requests\n\nThe main feature of this interface is accurate presentation\ntiming feedback to ensure smooth video playback while maintaining\naudio/video synchronization. Some features use the concept of a\npresentation clock, which is defined in the\npresentation.clock_id event.\n\nA content update for a wl_surface is submitted by a\nwl_surface.commit request. Request 'feedback' associates with\nthe wl_surface.commit and provides feedback on the content\nupdate, particularly the final realized presentation time.\n\n\n\nWhen the final realized presentation time is available, e.g.\nafter a framebuffer flip completes, the requested\npresentation_feedback.presented events are sent. The final\npresentation time can differ from the compositor's predicted\ndisplay update time and the update's target time, especially\nwhen the compositor misses its target vertical blanking period." ] pub mod wp_presentation { use super :: { Proxy , NewProxy , AnonymousObject , Interface , MessageGroup , MessageDesc , ArgumentType , Object , Message , Argument , ObjectMetadata } ; use super :: sys :: common :: { wl_argument , wl_interface , wl_array } ; use super :: sys :: client :: * ; # [ doc = "fatal presentation errors\n\nThese fatal protocol errors may be emitted in response to\nillegal presentation requests." ] # [ repr ( u32 ) ] # [ derive ( Copy , Clone , Debug , PartialEq ) ] pub enum Error { # [ doc = "invalid value in tv_nsec" ] InvalidTimestamp = 0 , # [ doc = "invalid flag" ] InvalidFlag = 1 , } impl Error { pub fn from_raw ( n : u32 ) -> Option < Error > { match n { 0 => Some ( Error :: InvalidTimestamp ) , 1 => Some ( Error :: InvalidFlag ) , _ => Option :: None } } pub fn to_raw ( & self ) -> u32 { * self as u32 } } pub enum Request { # [ doc = "unbind from the presentation interface\n\nInforms the server that the client will no longer be using\nthis protocol object. Existing objects created by this object\nare not affected.\n\nThis is a destructor, once sent this object cannot be used any longer." ] Destroy , # [ doc = "request presentation feedback information\n\nRequest presentation feedback for the current content submission\non the given surface. This creates a new presentation_feedback\nobject, which will deliver the feedback information once. If\nmultiple presentation_feedback objects are created for the same\nsubmission, they will all deliver the same information.\n\nFor details on what information is returned, see the\npresentation_feedback interface." ] Feedback { surface : Proxy < super :: wl_surface :: WlSurface > , callback : Proxy < super :: wp_presentation_feedback :: WpPresentationFeedback > , } , } impl super :: MessageGroup for Request { const MESSAGES : & 'static [ super :: MessageDesc ] = & [ super :: MessageDesc { name : "destroy" , since : 1 , signature : & [ ] , } , super :: MessageDesc { name : "feedback" , since : 1 , signature : & [ super :: ArgumentType :: Object , super :: ArgumentType :: NewId , ] , } , ] ; type Map = super :: ProxyMap ; fn is_destructor ( & self ) -> bool { match * self { Request :: Destroy => true , _ => false , } } fn opcode ( & self ) -> u16 { match * self { Request :: Destroy => 0 , Request :: Feedback { .. } => 1 , } } fn child < Meta : ObjectMetadata > ( opcode : u16 , version : u32 , meta : & Meta ) -> Option < Object < Meta >> { match opcode { 1 => Some ( Object :: from_interface :: < super :: wp_presentation_feedback :: WpPresentationFeedback > ( version , meta . child ( ) , ) ) , _ => None , } } fn from_raw ( msg : Message , map : & mut Self :: Map ) -> Result < Self , ( ) > { panic ! ( "Request::from_raw can not be used Client-side." ) } fn into_raw ( self , sender_id : u32 ) -> Message { match self { Request :: Destroy => Message { sender_id : sender_id , opcode : 0 , args : vec ! [ ] , } , Request :: Feedback { surface , callback } => Message { sender_id : sender_id , opcode : 1 , args : vec ! [ Argument :: Object ( surface . id ( ) ) , Argument :: NewId ( callback . id ( ) ) , ] , } , } } unsafe fn from_raw_c ( obj : * mut :: std :: os :: raw :: c_void , opcode : u32 , args : * const wl_argument , ) -> Result < Request , ( ) > { panic ! ( "Request::from_raw_c can not be used Client-side." ) } fn as_raw_c_in < F , T > ( self , f : F ) -> T where F : FnOnce ( u32 , & mut [ wl_argument ] ) -> T { match self { Request :: Destroy => { let mut _args_array : [ wl_argument ; 0 ] = unsafe { :: std :: mem :: zeroed ( ) } ; f ( 0 , & mut _args_array ) } , Request :: Feedback { surface , callback } => { let mut _args_array : [ wl_argument ; 2 ] = unsafe { :: std :: mem :: zeroed ( ) } ; _args_array [ 0 ] . o = surface . c_ptr ( ) as * mut _ ; _args_array [ 1 ] . o = callback . c_ptr ( ) as * mut _ ; f ( 1 , & mut _args_array ) } , } } } pub enum Event { # [ doc = "clock ID for timestamps\n\nThis event tells the client in which clock domain the\ncompositor interprets the timestamps used by the presentation\nextension. This clock is called the presentation clock.\n\nThe compositor sends this event when the client binds to the\npresentation interface. The presentation clock does not change\nduring the lifetime of the client connection.\n\nThe clock identifier is platform dependent. On Linux/glibc,\nthe identifier value is one of the clockid_t values accepted\nby clock_gettime(). clock_gettime() is defined by\nPOSIX.1-2001.\n\nTimestamps in this clock domain are expressed as tv_sec_hi,\ntv_sec_lo, tv_nsec triples, each component being an unsigned\n32-bit value. Whole seconds are in tv_sec which is a 64-bit\nvalue combined from tv_sec_hi and tv_sec_lo, and the\nadditional fractional part in tv_nsec as nanoseconds. Hence,\nfor valid timestamps tv_nsec must be in [0, 999999999].\n\nNote that clock_id applies only to the presentation clock,\nand implies nothing about e.g. the timestamps used in the\nWayland core protocol input events.\n\nCompositors should prefer a clock which does not jump and is\nnot slewed e.g. by NTP. The absolute value of the clock is\nirrelevant. Precision of one millisecond or better is\nrecommended. Clients must be able to query the current clock\nvalue directly, not by asking the compositor." ] ClockId { clk_id : u32 , } , } impl super :: MessageGroup for Event { const MESSAGES : & 'static [ super :: MessageDesc ] = & [ super :: MessageDesc { name : "clock_id" , since : 1 , signature : & [ super :: ArgumentType :: Uint , ] , } , ] ; type Map = super :: ProxyMap ; fn is_destructor ( & self ) -> bool { match * self { _ => false , } } fn opcode ( & self ) -> u16 { match * self { Event :: ClockId { .. } => 0 , } } fn child < Meta : ObjectMetadata > ( opcode : u16 , version : u32 , meta : & Meta ) -> Option < Object < Meta >> { match opcode { _ => None , } } fn from_raw ( msg : Message , map : & mut Self :: Map ) -> Result < Self , ( ) > { match msg . opcode { 0 => { let mut args = msg . args . into_iter ( ) ; Ok ( Event :: ClockId { clk_id : { if let Some ( Argument :: Uint ( val ) ) = args . next ( ) { val } else { return Err ( ( ) ) ; } } , } ) } , _ => Err ( ( ) ) , } } fn into_raw ( self , sender_id : u32 ) -> Message { panic ! ( "Event::into_raw can not be used Client-side." ) } unsafe fn from_raw_c ( obj : * mut :: std :: os :: raw :: c_void , opcode : u32 , args : * const wl_argument , ) -> Result < Event , ( ) > { match opcode { 0 => { let _args = :: std :: slice :: from_raw_parts ( args , 1 ) ; Ok ( Event :: ClockId { clk_id : _args [ 0 ] . u , } ) } , _ => return Err ( ( ) ) , } } fn as_raw_c_in < F , T > ( self , f : F ) -> T where F : FnOnce ( u32 , & mut [ wl_argument ] ) -> T { panic ! ( "Event::as_raw_c_in can not be used Client-side." ) } } pub struct WpPresentation ; impl Interface for WpPresentation { type Request = Request ; type Event = Event ; const NAME : & 'static str = "wp_presentation" ; const VERSION : u32 = 1 ; fn c_interface ( ) -> * const wl_interface { unsafe { & super :: super :: c_interfaces :: wp_presentation_interface } } } pub trait RequestsTrait { # [ doc = "unbind from the presentation interface\n\nInforms the server that the client will no longer be using\nthis protocol object. Existing objects created by this object\nare not affected.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called." ] fn destroy ( & self , ) -> ( ) ; # [ doc = "request presentation feedback information\n\nRequest presentation feedback for the current content submission\non the given surface. This creates a new presentation_feedback\nobject, which will deliver the feedback information once. If\nmultiple presentation_feedback objects are created for the same\nsubmission, they will all deliver the same information.\n\nFor details on what information is returned, see the\npresentation_feedback interface." ] fn feedback < F > ( & self , surface : & Proxy < super :: wl_surface :: WlSurface > , implementor : F ) -> Result < Proxy < super :: wp_presentation_feedback :: WpPresentationFeedback > , ( ) > where F : FnOnce ( NewProxy < super :: wp_presentation_feedback :: WpPresentationFeedback > , ) -> Proxy < super :: wp_presentation_feedback :: WpPresentationFeedback > ; } impl RequestsTrait for Proxy < WpPresentation > { fn destroy ( & self , ) -> ( ) { let msg = Request :: Destroy ; self . send ( msg ) ; } fn feedback < F > ( & self , surface : & Proxy < super :: wl_surface :: WlSurface > , implementor : F ) -> Result < Proxy < super :: wp_presentation_feedback :: WpPresentationFeedback > , ( ) > where F : FnOnce ( NewProxy < super :: wp_presentation_feedback :: WpPresentationFeedback > , ) -> Proxy < super :: wp_presentation_feedback :: WpPresentationFeedback > { let msg = Request :: Feedback { surface : surface . clone ( ) , callback : self . child_placeholder ( ) } ; self . send_constructor ( msg , implementor , None ) } } # [ doc = r" The minimal object version supporting this request" ] pub const REQ_DESTROY_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this request" ] pub const REQ_FEEDBACK_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this event" ] pub const EVT_CLOCK_ID_SINCE : u16 = 1u16 ; } # [ doc = "presentation time feedback event\n\nA presentation_feedback object returns an indication that a\nwl_surface content update has become visible to the user.\nOne object corresponds to one content update submission\n(wl_surface.commit). There are two possible outcomes: the\ncontent update is presented to the user, and a presentation\ntimestamp delivered; or, the user did not see the content\nupdate because it was superseded or its surface destroyed,\nand the content update is discarded.\n\nOnce a presentation_feedback object has delivered a 'presented'\nor 'discarded' event it is automatically destroyed." ] pub mod wp_presentation_feedback { use super :: { Proxy , NewProxy , AnonymousObject , Interface , MessageGroup , MessageDesc , ArgumentType , Object , Message , Argument , ObjectMetadata } ; use super :: sys :: common :: { wl_argument , wl_interface , wl_array } ; use super :: sys :: client :: * ; # [ doc = "bitmask of flags in presented event\n\nThese flags provide information about how the presentation of\nthe related content update was done. The intent is to help\nclients assess the reliability of the feedback and the visual\nquality with respect to possible tearing and timings. The\nflags are:\n\nVSYNC:\nThe presentation was synchronized to the \"vertical retrace\" by\nthe display hardware such that tearing does not happen.\nRelying on user space scheduling is not acceptable for this\nflag. If presentation is done by a copy to the active\nfrontbuffer, then it must guarantee that tearing cannot\nhappen.\n\nHW_CLOCK:\nThe display hardware provided measurements that the hardware\ndriver converted into a presentation timestamp. Sampling a\nclock in user space is not acceptable for this flag.\n\nHW_COMPLETION:\nThe display hardware signalled that it started using the new\nimage content. The opposite of this is e.g. a timer being used\nto guess when the display hardware has switched to the new\nimage content.\n\nZERO_COPY:\nThe presentation of this update was done zero-copy. This means\nthe buffer from the client was given to display hardware as\nis, without copying it. Compositing with OpenGL counts as\ncopying, even if textured directly from the client buffer.\nPossible zero-copy cases include direct scanout of a\nfullscreen surface and a surface on a hardware overlay." ] # [ repr ( u32 ) ] # [ derive ( Copy , Clone , Debug , PartialEq ) ] pub enum Kind { # [ doc = "presentation was vsync'd" ] Vsync = 1 , # [ doc = "hardware provided the presentation timestamp" ] HwClock = 2 , # [ doc = "hardware signalled the start of the presentation" ] HwCompletion = 4 , # [ doc = "presentation was done zero-copy" ] ZeroCopy = 8 , } impl Kind { pub fn from_raw ( n : u32 ) -> Option < Kind > { match n { 1 => Some ( Kind :: Vsync ) , 2 => Some ( Kind :: HwClock ) , 4 => Some ( Kind :: HwCompletion ) , 8 => Some ( Kind :: ZeroCopy ) , _ => Option :: None } } pub fn to_raw ( & self ) -> u32 { * self as u32 } } pub enum Request { } impl super :: MessageGroup for Request { const MESSAGES : & 'static [ super :: MessageDesc ] = & [ ] ; type Map = super :: ProxyMap ; fn is_destructor ( & self ) -> bool { match * self { } } fn opcode ( & self ) -> u16 { match * self { } } fn child < Meta : ObjectMetadata > ( opcode : u16 , version : u32 , meta : & Meta ) -> Option < Object < Meta >> { match opcode { _ => None , } } fn from_raw ( msg : Message , map : & mut Self :: Map ) -> Result < Self , ( ) > { panic ! ( "Request::from_raw can not be used Client-side." ) } fn into_raw ( self , sender_id : u32 ) -> Message { match self { } } unsafe fn from_raw_c ( obj : * mut :: std :: os :: raw :: c_void , opcode : u32 , args : * const wl_argument , ) -> Result < Request , ( ) > { panic ! ( "Request::from_raw_c can not be used Client-side." ) } fn as_raw_c_in < F , T > ( self , f : F ) -> T where F : FnOnce ( u32 , & mut [ wl_argument ] ) -> T { match self { } } } pub enum Event { # [ doc = "presentation synchronized to this output\n\nAs presentation can be synchronized to only one output at a\ntime, this event tells which output it was. This event is only\nsent prior to the presented event.\n\nAs clients may bind to the same global wl_output multiple\ntimes, this event is sent for each bound instance that matches\nthe synchronized output. If a client has not bound to the\nright wl_output global at all, this event is not sent." ] SyncOutput { output : Proxy < super :: wl_output :: WlOutput > , } , # [ doc = "the content update was displayed\n\nThe associated content update was displayed to the user at the\nindicated time (tv_sec_hi/lo, tv_nsec). For the interpretation of\nthe timestamp, see presentation.clock_id event.\n\nThe timestamp corresponds to the time when the content update\nturned into light the first time on the surface's main output.\nCompositors may approximate this from the framebuffer flip\ncompletion events from the system, and the latency of the\nphysical display path if known.\n\nThis event is preceded by all related sync_output events\ntelling which output's refresh cycle the feedback corresponds\nto, i.e. the main output for the surface. Compositors are\nrecommended to choose the output containing the largest part\nof the wl_surface, or keeping the output they previously\nchose. Having a stable presentation output association helps\nclients predict future output refreshes (vblank).\n\nThe 'refresh' argument gives the compositor's prediction of how\nmany nanoseconds after tv_sec, tv_nsec the very next output\nrefresh may occur. This is to further aid clients in\npredicting future refreshes, i.e., estimating the timestamps\ntargeting the next few vblanks. If such prediction cannot\nusefully be done, the argument is zero.\n\nIf the output does not have a constant refresh rate, explicit\nvideo mode switches excluded, then the refresh argument must\nbe zero.\n\nThe 64-bit value combined from seq_hi and seq_lo is the value\nof the output's vertical retrace counter when the content\nupdate was first scanned out to the display. This value must\nbe compatible with the definition of MSC in\nGLX_OML_sync_control specification. Note, that if the display\npath has a non-zero latency, the time instant specified by\nthis counter may differ from the timestamp's.\n\nIf the output does not have a concept of vertical retrace or a\nrefresh cycle, or the output device is self-refreshing without\na way to query the refresh count, then the arguments seq_hi\nand seq_lo must be zero." ] Presented { tv_sec_hi : u32 , tv_sec_lo : u32 , tv_nsec : u32 , refresh : u32 , seq_hi : u32 , seq_lo : u32 , flags : u32 , } , # [ doc = "the content update was not displayed\n\nThe content update was never displayed to the user." ] Discarded , } impl super :: MessageGroup for Event { const MESSAGES : & 'static [ super :: MessageDesc ] = & [ super :: MessageDesc { name : "sync_output" , since : 1 , signature : & [ super :: ArgumentType :: Object , ] , } , super :: MessageDesc { name : "presented" , since : 1 , signature : & [ super :: ArgumentType :: Uint , super :: ArgumentType :: Uint , super :: ArgumentType :: Uint , super :: ArgumentType :: Uint , super :: ArgumentType :: Uint , super :: ArgumentType :: Uint , super :: ArgumentType :: Uint , ] , } , super :: MessageDesc { name : "discarded" , since : 1 , signature : & [ ] , } , ] ; type Map = super :: ProxyMap ; fn is_destructor ( & self ) -> bool { match * self { _ => false , } } fn opcode ( & self ) -> u16 { match * self { Event :: SyncOutput { .. } => 0 , Event :: Presented { .. } => 1 , Event :: Discarded => 2 , } } fn child < Meta : ObjectMetadata > ( opcode : u16 , version : u32 , meta : & Meta ) -> Option < Object < Meta >> { match opcode { _ => None , } } fn from_raw ( msg : Message , map : & mut Self :: Map ) -> Result < Self , ( ) > { match msg . opcode { 0 => { let mut args = msg . args . into_iter ( ) ; Ok ( Event :: SyncOutput { output : { if let Some ( Argument :: Object ( val ) ) = args . next ( ) { map . get ( val ) . ok_or ( ( ) ) ? } else { return Err ( ( ) ) ; } } , } ) } , 1 => { let mut args = msg . args . into_iter ( ) ; Ok ( Event :: Presented { tv_sec_hi : { if let Some ( Argument :: Uint ( val ) ) = args . next ( ) { val } else { return Err ( ( ) ) ; } } , tv_sec_lo : { if let Some ( Argument :: Uint ( val ) ) = args . next ( ) { val } else { return Err ( ( ) ) ; } } , tv_nsec : { if let Some ( Argument :: Uint ( val ) ) = args . next ( ) { val } else { return Err ( ( ) ) ; } } , refresh : { if let Some ( Argument :: Uint ( val ) ) = args . next ( ) { val } else { return Err ( ( ) ) ; } } , seq_hi : { if let Some ( Argument :: Uint ( val ) ) = args . next ( ) { val } else { return Err ( ( ) ) ; } } , seq_lo : { if let Some ( Argument :: Uint ( val ) ) = args . next ( ) { val } else { return Err ( ( ) ) ; } } , flags : { if let Some ( Argument :: Uint ( val ) ) = args . next ( ) { val } else { return Err ( ( ) ) ; } } , } ) } , 2 => Ok ( Event :: Discarded ) , _ => Err ( ( ) ) , } } fn into_raw ( self , sender_id : u32 ) -> Message { panic ! ( "Event::into_raw can not be used Client-side." ) } unsafe fn from_raw_c ( obj : * mut :: std :: os :: raw :: c_void , opcode : u32 , args : * const wl_argument , ) -> Result < Event , ( ) > { match opcode { 0 => { let _args = :: std :: slice :: from_raw_parts ( args , 1 ) ; Ok ( Event :: SyncOutput { output : Proxy :: < super :: wl_output :: WlOutput > :: from_c_ptr ( _args [ 0 ] . o as * mut _ , ) , } ) } , 1 => { let _args = :: std :: slice :: from_raw_parts ( args , 7 ) ; Ok ( Event :: Presented { tv_sec_hi : _args [ 0 ] . u , tv_sec_lo : _args [ 1 ] . u , tv_nsec : _args [ 2 ] . u , refresh : _args [ 3 ] . u , seq_hi : _args [ 4 ] . u , seq_lo : _args [ 5 ] . u , flags : _args [ 6 ] . u , } ) } , 2 => { Ok ( Event :: Discarded ) } , _ => return Err ( ( ) ) , } } fn as_raw_c_in < F , T > ( self , f : F ) -> T where F : FnOnce ( u32 , & mut [ wl_argument ] ) -> T { panic ! ( "Event::as_raw_c_in can not be used Client-side." ) } } pub struct WpPresentationFeedback ; impl Interface for WpPresentationFeedback { type Request = Request ; type Event = Event ; const NAME : & 'static str = "wp_presentation_feedback" ; const VERSION : u32 = 1 ; fn c_interface ( ) -> * const wl_interface { unsafe { & super :: super :: c_interfaces :: wp_presentation_feedback_interface } } } pub trait RequestsTrait { } impl RequestsTrait for Proxy < WpPresentationFeedback > { } # [ doc = r" The minimal object version supporting this event" ] pub const EVT_SYNC_OUTPUT_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this event" ] pub const EVT_PRESENTED_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this event" ] pub const EVT_DISCARDED_SINCE : u16 = 1u16 ; }