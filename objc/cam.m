#import <AVFoundation/AVFoundation.h>
#import <AppKit/AppKit.h>

@interface Capture : NSObject<AVCaptureVideoDataOutputSampleBufferDelegate>
@property (atomic, retain) AVCaptureSession *session;
@property (atomic, retain) NSData *jpgData;
+ (Capture *)sharedInstance;
- (void)captureOutput:(AVCaptureOutput *)output
    didOutputSampleBuffer:(CMSampleBufferRef)buffer
           fromConnection:(AVCaptureConnection *)connection;
@end

@implementation Capture
static Capture *_sharedInstance = nil;

+ (Capture *)sharedInstance {
  static dispatch_once_t onceToken;
  dispatch_once(&onceToken, ^{
    _sharedInstance = [[Capture alloc] init];
  });
  return _sharedInstance;
}

- (void)captureOutput:(AVCaptureOutput *)output
    didOutputSampleBuffer:(CMSampleBufferRef)buffer
           fromConnection:(AVCaptureConnection *)connection {
  @autoreleasepool {
    CIImage *ciImage =
        [CIImage imageWithCVImageBuffer:CMSampleBufferGetImageBuffer(buffer)];
    NSBitmapImageRep *bitmapRep =
        [[NSBitmapImageRep alloc] initWithCIImage:ciImage];
    NSData *jpgData =
        [bitmapRep representationUsingType:NSJPEGFileType
                                properties:@{}];
    [[Capture sharedInstance] setValue:jpgData forKey:@"jpgData"];
    [[NSRunLoop currentRunLoop] runUntilDate:[NSDate dateWithTimeIntervalSinceNow:0.033]];
  }
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
  }
}

NSData *get_last_capture() {
  return [Capture sharedInstance].jpgData;
}