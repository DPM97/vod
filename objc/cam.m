#import <AVFoundation/AVFoundation.h>
#import <AppKit/AppKit.h>

@interface Capture : NSObject<AVCaptureVideoDataOutputSampleBufferDelegate>
@property (nonatomic, retain) AVCaptureSession *session;
+ (Capture *)sharedInstance;
- (void)captureOutput:(AVCaptureOutput *)output
    didOutputSampleBuffer:(CMSampleBufferRef)buffer
           fromConnection:(AVCaptureConnection *)connection;
@end

@interface Capture () {
  CVImageBufferRef head;
  CFRunLoopRef runLoop;
  int count;
}
@property (nonatomic, retain) NSData *jpgData;
@end

@implementation Capture
@synthesize session;

static Capture *_sharedInstance = nil;

+ (Capture *)sharedInstance {
  static dispatch_once_t onceToken;
  dispatch_once(&onceToken, ^{
    _sharedInstance = [[Capture alloc] init];
  });

  return _sharedInstance;
}

- (id)init {
  self = [super init];
  runLoop = CFRunLoopGetCurrent();
  head = nil;
  return self;
}

- (void)dealloc {
  @synchronized(self) {
    CVBufferRelease(head);
  }
  [super dealloc];
  NSLog(@"capture released");
}

- (void)captureOutput:(AVCaptureOutput *)output
    didOutputSampleBuffer:(CMSampleBufferRef)buffer
           fromConnection:(AVCaptureConnection *)connection {
  @autoreleasepool {
    [[Capture sharedInstance] setValue:[[[NSBitmapImageRep alloc] initWithCIImage:[CIImage imageWithCVImageBuffer:CMSampleBufferGetImageBuffer(buffer)]] representationUsingType:NSJPEGFileType
                                                                                                                                                                      properties:@{}]
                                forKey:@"jpgData"];
  }
  // wait so that the capture rate is about 60fps
  [[NSRunLoop currentRunLoop] runUntilDate:[NSDate dateWithTimeIntervalSinceNow:0.033]];
  // CFRunLoopStop(runLoop);
}

@end

void start_capture_loop() {
  @autoreleasepool {
    NSError *error = nil;
    Capture *capture = [[Capture alloc] init];

    AVCaptureDevice *device =
        [AVCaptureDevice defaultDeviceWithMediaType:AVMediaTypeVideo];
    NSLog(@"[Device] %@", device);

    AVCaptureDeviceInput *input =
        [AVCaptureDeviceInput deviceInputWithDevice:device
                                              error:&error];
    NSLog(@"[Input] %@", input);

    AVCaptureVideoDataOutput *output =
        [[AVCaptureVideoDataOutput alloc] init];
    [output setSampleBufferDelegate:capture queue:dispatch_queue_create("Lots of requests", NULL)];
    NSLog(@"[Output] %@", output);

    AVCaptureSession *session = [[AVCaptureSession alloc] init];
    [session addInput:input];
    [session addOutput:output];

    capture.session = session;
    [session startRunning];
    NSLog(@"Started Capture");
    CFRunLoopRun();
  }
}

NSData *get_last_capture() {
  return [Capture sharedInstance].jpgData;
}