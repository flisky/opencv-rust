//! # Core functionality
//! # Basic structures
//! # C structures and operations
//! # Connections with C++
//! # Operations on arrays
//! # XML/YAML Persistence
//! # Clustering
//! # Utility and system functions and macros
//! # SSE utilities
//! # NEON utilities
//! # Softfloat support
//! # Utility functions for OpenCV samples
//! # OpenGL interoperability
//! # Intel IPP Asynchronous C/C++ Converters
//! # Optimization Algorithms
//! # DirectX interoperability
//! # Eigen support
//! # OpenCL support
//! # Intel VA-API/OpenCL (CL-VA) interoperability
//! # Hardware Acceleration Layer
//! # Functions
//! # Interface
//! # Universal intrinsics
//! # Private implementation helpers
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};

pub const ACCESS_FAST: i32 = 1<<26;
pub const ACCESS_MASK: i32 = 3<<24;
pub const ACCESS_READ: i32 = 1<<24;
pub const ACCESS_RW: i32 = 3<<24;
pub const ACCESS_WRITE: i32 = 1<<25;
pub const BORDER_CONSTANT: i32 = 0;
pub const BORDER_DEFAULT: i32 = 4;
pub const BORDER_ISOLATED: i32 = 16;
pub const BORDER_REFLECT: i32 = 2;
pub const BORDER_REFLECT101: i32 = 4;
pub const BORDER_REFLECT_101: i32 = 4;
pub const BORDER_REPLICATE: i32 = 1;
pub const BORDER_TRANSPARENT: i32 = 5;
pub const BORDER_WRAP: i32 = 3;
pub const BadAlign: i32 = -21;
pub const BadAlphaChannel: i32 = -18;
pub const BadCOI: i32 = -24;
pub const BadCallBack: i32 = -22;
pub const BadDataPtr: i32 = -12;
pub const BadDepth: i32 = -17;
pub const BadImageSize: i32 = -10;
pub const BadModelOrChSeq: i32 = -14;
pub const BadNumChannel1U: i32 = -16;
pub const BadNumChannels: i32 = -15;
pub const BadOffset: i32 = -11;
pub const BadOrder: i32 = -19;
pub const BadOrigin: i32 = -20;
pub const BadROISize: i32 = -25;
pub const BadStep: i32 = -13;
pub const BadTileSize: i32 = -23;
pub const CMP_EQ: i32 = 0;
pub const CMP_GE: i32 = 2;
pub const CMP_GT: i32 = 1;
pub const CMP_LE: i32 = 4;
pub const CMP_LT: i32 = 3;
pub const CMP_NE: i32 = 5;
pub const COVAR_COLS: i32 = 16;
pub const COVAR_NORMAL: i32 = 1;
pub const COVAR_ROWS: i32 = 8;
pub const COVAR_SCALE: i32 = 4;
pub const COVAR_SCRAMBLED: i32 = 0;
pub const COVAR_USE_AVG: i32 = 2;
pub const CPU_AVX: i32 = 10;
pub const CPU_AVX2: i32 = 11;
pub const CPU_AVX512_SKX: i32 = 256;
pub const CPU_AVX_512BW: i32 = 14;
pub const CPU_AVX_512CD: i32 = 15;
pub const CPU_AVX_512DQ: i32 = 16;
pub const CPU_AVX_512ER: i32 = 17;
pub const CPU_AVX_512F: i32 = 13;
pub const CPU_AVX_512IFMA: i32 = 18;
pub const CPU_AVX_512IFMA512: i32 = 18;
pub const CPU_AVX_512PF: i32 = 19;
pub const CPU_AVX_512VBMI: i32 = 20;
pub const CPU_AVX_512VL: i32 = 21;
pub const CPU_FMA3: i32 = 12;
pub const CPU_FP16: i32 = 9;
pub const CPU_MAX_FEATURE: i32 = 512;
pub const CPU_MMX: i32 = 1;
pub const CPU_NEON: i32 = 100;
pub const CPU_POPCNT: i32 = 8;
pub const CPU_SSE: i32 = 2;
pub const CPU_SSE2: i32 = 3;
pub const CPU_SSE3: i32 = 4;
pub const CPU_SSE4_1: i32 = 6;
pub const CPU_SSE4_2: i32 = 7;
pub const CPU_SSSE3: i32 = 5;
pub const CPU_VSX: i32 = 200;
pub const CPU_VSX3: i32 = 201;
pub const CV_16S: i32 = 3;
pub const CV_16U: i32 = 2;
pub const CV_32F: i32 = 5;
pub const CV_32S: i32 = 4;
pub const CV_64F: i32 = 6;
pub const CV_8S: i32 = 1;
pub const CV_8U: i32 = 0;
pub const CV_CN_MAX: i32 = 512;
pub const CV_CN_SHIFT: i32 = 3;
pub const CV_CPU_AVX: i32 = 10;
pub const CV_CPU_AVX2: i32 = 11;
pub const CV_CPU_AVX512_SKX: i32 = 256;
pub const CV_CPU_AVX_512BW: i32 = 14;
pub const CV_CPU_AVX_512CD: i32 = 15;
pub const CV_CPU_AVX_512DQ: i32 = 16;
pub const CV_CPU_AVX_512ER: i32 = 17;
pub const CV_CPU_AVX_512F: i32 = 13;
pub const CV_CPU_AVX_512IFMA: i32 = 18;
/// deprecated
pub const CV_CPU_AVX_512IFMA512: i32 = 18;
pub const CV_CPU_AVX_512PF: i32 = 19;
pub const CV_CPU_AVX_512VBMI: i32 = 20;
pub const CV_CPU_AVX_512VL: i32 = 21;
pub const CV_CPU_FMA3: i32 = 12;
pub const CV_CPU_FP16: i32 = 9;
pub const CV_CPU_MMX: i32 = 1;
pub const CV_CPU_NEON: i32 = 100;
pub const CV_CPU_NONE: i32 = 0;
pub const CV_CPU_POPCNT: i32 = 8;
pub const CV_CPU_SSE: i32 = 2;
pub const CV_CPU_SSE2: i32 = 3;
pub const CV_CPU_SSE3: i32 = 4;
pub const CV_CPU_SSE4_1: i32 = 6;
pub const CV_CPU_SSE4_2: i32 = 7;
pub const CV_CPU_SSSE3: i32 = 5;
pub const CV_CPU_VSX: i32 = 200;
pub const CV_CPU_VSX3: i32 = 201;
pub const CV_HAL_BORDER_CONSTANT: i32 = 0;
pub const CV_HAL_BORDER_ISOLATED: i32 = 16;
pub const CV_HAL_BORDER_REFLECT: i32 = 2;
pub const CV_HAL_BORDER_REFLECT_101: i32 = 4;
pub const CV_HAL_BORDER_REPLICATE: i32 = 1;
pub const CV_HAL_BORDER_TRANSPARENT: i32 = 5;
pub const CV_HAL_BORDER_WRAP: i32 = 3;
pub const CV_HAL_CMP_EQ: i32 = 0;
pub const CV_HAL_CMP_GE: i32 = 2;
pub const CV_HAL_CMP_GT: i32 = 1;
pub const CV_HAL_CMP_LE: i32 = 4;
pub const CV_HAL_CMP_LT: i32 = 3;
pub const CV_HAL_CMP_NE: i32 = 5;
pub const CV_HAL_DFT_COMPLEX_OUTPUT: i32 = 16;
pub const CV_HAL_DFT_INVERSE: i32 = 1;
pub const CV_HAL_DFT_IS_CONTINUOUS: i32 = 512;
pub const CV_HAL_DFT_IS_INPLACE: i32 = 1024;
pub const CV_HAL_DFT_REAL_OUTPUT: i32 = 32;
pub const CV_HAL_DFT_ROWS: i32 = 4;
pub const CV_HAL_DFT_SCALE: i32 = 2;
pub const CV_HAL_DFT_STAGE_COLS: i32 = 128;
pub const CV_HAL_DFT_TWO_STAGE: i32 = 64;
pub const CV_HAL_ERROR_NOT_IMPLEMENTED: i32 = 1;
pub const CV_HAL_ERROR_OK: i32 = 0;
pub const CV_HAL_ERROR_UNKNOWN: i32 = -1;
pub const CV_HAL_GEMM_1_T: i32 = 1;
pub const CV_HAL_GEMM_2_T: i32 = 2;
pub const CV_HAL_GEMM_3_T: i32 = 4;
pub const CV_HAL_SVD_FULL_UV: i32 = 8;
pub const CV_HAL_SVD_MODIFY_A: i32 = 4;
pub const CV_HAL_SVD_NO_UV: i32 = 1;
pub const CV_HAL_SVD_SHORT_UV: i32 = 2;
pub const CV_HARDWARE_MAX_FEATURE: i32 = 512;
/// !< Debug message. Disabled in the "Release" build.
pub const CV_LOG_LEVEL_DEBUG: i32 = 5;
/// !< Error message
pub const CV_LOG_LEVEL_ERROR: i32 = 2;
/// !< Fatal (critical) error (unrecoverable internal error)
pub const CV_LOG_LEVEL_FATAL: i32 = 1;
/// !< Info message
pub const CV_LOG_LEVEL_INFO: i32 = 4;
/// !< for using in setLogLevel() call
pub const CV_LOG_LEVEL_SILENT: i32 = 0;
/// !< Verbose (trace) messages. Requires verbosity level. Disabled in the "Release" build.
pub const CV_LOG_LEVEL_VERBOSE: i32 = 6;
/// !< Warning message
pub const CV_LOG_LEVEL_WARN: i32 = 3;
pub const CV_MAJOR_VERSION: i32 = 3;
pub const CV_MAT_CONT_FLAG_SHIFT: i32 = 14;
pub const CV_MINOR_VERSION: i32 = 4;
pub const CV_SUBMAT_FLAG_SHIFT: i32 = 15;
pub const CV_SUBMINOR_VERSION: i32 = 6;
pub const CV_USRTYPE1: i32 = 7;
pub const CV_VERSION_MAJOR: i32 = 3;
pub const CV_VERSION_MINOR: i32 = 4;
pub const CV_VERSION_REVISION: i32 = 6;
pub const CV_VERSION_STATUS: &'static str = "";
pub const DCT_INVERSE: i32 = 1;
pub const DCT_ROWS: i32 = 4;
pub const DECOMP_CHOLESKY: i32 = 3;
pub const DECOMP_EIG: i32 = 2;
pub const DECOMP_LU: i32 = 0;
pub const DECOMP_NORMAL: i32 = 16;
pub const DECOMP_QR: i32 = 4;
pub const DECOMP_SVD: i32 = 1;
pub const DFT_COMPLEX_INPUT: i32 = 64;
pub const DFT_COMPLEX_OUTPUT: i32 = 16;
pub const DFT_INVERSE: i32 = 1;
pub const DFT_REAL_OUTPUT: i32 = 32;
pub const DFT_ROWS: i32 = 4;
pub const DFT_SCALE: i32 = 2;
pub const FILLED: i32 = -1;
pub const FLAGS_EXPAND_SAME_NAMES: i32 = 0x02;
pub const FLAGS_MAPPING: i32 = 0x01;
pub const FLAGS_NONE: i32 = 0;
pub const FONT_HERSHEY_COMPLEX: i32 = 3;
pub const FONT_HERSHEY_COMPLEX_SMALL: i32 = 5;
pub const FONT_HERSHEY_DUPLEX: i32 = 2;
pub const FONT_HERSHEY_PLAIN: i32 = 1;
pub const FONT_HERSHEY_SCRIPT_COMPLEX: i32 = 7;
pub const FONT_HERSHEY_SCRIPT_SIMPLEX: i32 = 6;
pub const FONT_HERSHEY_SIMPLEX: i32 = 0;
pub const FONT_HERSHEY_TRIPLEX: i32 = 4;
pub const FONT_ITALIC: i32 = 16;
pub const Formatter_FMT_C: i32 = 5;
pub const Formatter_FMT_CSV: i32 = 2;
pub const Formatter_FMT_DEFAULT: i32 = 0;
pub const Formatter_FMT_MATLAB: i32 = 1;
pub const Formatter_FMT_NUMPY: i32 = 4;
pub const Formatter_FMT_PYTHON: i32 = 3;
pub const GEMM_1_T: i32 = 1;
pub const GEMM_2_T: i32 = 2;
pub const GEMM_3_T: i32 = 4;
pub const GpuApiCallError: i32 = -217;
pub const GpuNotSupported: i32 = -216;
pub const Hamming_normType: i32 = 6;
pub const HeaderIsNull: i32 = -9;
pub const IMPL_IPP: i32 = 0+1;
pub const IMPL_OPENCL: i32 = 0+2;
pub const IMPL_PLAIN: i32 = 0;
pub const KMEANS_PP_CENTERS: i32 = 2;
pub const KMEANS_RANDOM_CENTERS: i32 = 0;
pub const KMEANS_USE_INITIAL_LABELS: i32 = 1;
pub const LINE_4: i32 = 4;
pub const LINE_8: i32 = 8;
pub const LINE_AA: i32 = 16;
pub const LOG_LEVEL_DEBUG: i32 = 5;
pub const LOG_LEVEL_ERROR: i32 = 2;
pub const LOG_LEVEL_FATAL: i32 = 1;
pub const LOG_LEVEL_INFO: i32 = 4;
pub const LOG_LEVEL_SILENT: i32 = 0;
pub const LOG_LEVEL_VERBOSE: i32 = 6;
pub const LOG_LEVEL_WARNING: i32 = 3;
pub const MaskIsTiled: i32 = -26;
pub const Mat_AUTO_STEP: usize = 0;
pub const Mat_DEPTH_MASK: i32 = 7;
pub const Mat_MAGIC_MASK: i32 = 0xFFFF0000;
pub const Mat_MAGIC_VAL: i32 = 0x42FF0000;
pub const Mat_TYPE_MASK: i32 = 0x00000FFF;
pub const NORM_HAMMING: i32 = 6;
pub const NORM_HAMMING2: i32 = 7;
pub const NORM_INF: i32 = 1;
pub const NORM_L1: i32 = 2;
pub const NORM_L2: i32 = 4;
pub const NORM_L2SQR: i32 = 5;
pub const NORM_MINMAX: i32 = 32;
pub const NORM_RELATIVE: i32 = 8;
pub const NORM_TYPE_MASK: i32 = 7;
pub const OPENCV_ABI_COMPATIBILITY: i32 = 300;
pub const OpenCLApiCallError: i32 = -220;
pub const OpenCLDoubleNotSupported: i32 = -221;
pub const OpenCLInitError: i32 = -222;
pub const OpenCLNoAMDBlasFft: i32 = -223;
pub const OpenGlApiCallError: i32 = -219;
pub const OpenGlNotSupported: i32 = -218;
pub const PCA_DATA_AS_COL: i32 = 1;
pub const PCA_DATA_AS_ROW: i32 = 0;
pub const PCA_USE_AVG: i32 = 2;
pub const Param_ALGORITHM: i32 = 6;
pub const Param_BOOLEAN: i32 = 1;
pub const Param_FLOAT: i32 = 7;
pub const Param_INT: i32 = 0;
pub const Param_MAT: i32 = 4;
pub const Param_MAT_VECTOR: i32 = 5;
pub const Param_REAL: i32 = 2;
pub const Param_SCALAR: i32 = 12;
pub const Param_STRING: i32 = 3;
pub const Param_UCHAR: i32 = 11;
pub const Param_UINT64: i32 = 9;
pub const Param_UNSIGNED_INT: i32 = 8;
pub const REDUCE_AVG: i32 = 1;
pub const REDUCE_MAX: i32 = 2;
pub const REDUCE_MIN: i32 = 3;
pub const REDUCE_SUM: i32 = 0;
pub const RNG_NORMAL: i32 = 1;
pub const RNG_UNIFORM: i32 = 0;
pub const ROTATE_180: i32 = 1;
pub const ROTATE_90_CLOCKWISE: i32 = 0;
pub const ROTATE_90_COUNTERCLOCKWISE: i32 = 2;
pub const SOLVELP_MULTI: i32 = 1;
pub const SOLVELP_SINGLE: i32 = 0;
pub const SOLVELP_UNBOUNDED: i32 = -2;
pub const SOLVELP_UNFEASIBLE: i32 = -1;
pub const SORT_ASCENDING: i32 = 0;
pub const SORT_DESCENDING: i32 = 16;
pub const SORT_EVERY_COLUMN: i32 = 1;
pub const SORT_EVERY_ROW: i32 = 0;
pub const SVD_FULL_UV: i32 = 4;
pub const SVD_MODIFY_A: i32 = 1;
pub const SVD_NO_UV: i32 = 2;
pub const SparseMat_HASH_BIT: i32 = 0x80000000;
pub const SparseMat_HASH_SCALE: i32 = 0x5bd1e995;
pub const SparseMat_MAX_DIM: i32 = 32;
pub const StsAssert: i32 = -215;
pub const StsAutoTrace: i32 = -8;
pub const StsBackTrace: i32 = -1;
pub const StsBadArg: i32 = -5;
pub const StsBadFlag: i32 = -206;
pub const StsBadFunc: i32 = -6;
pub const StsBadMask: i32 = -208;
pub const StsBadMemBlock: i32 = -214;
pub const StsBadPoint: i32 = -207;
pub const StsBadSize: i32 = -201;
pub const StsDivByZero: i32 = -202;
pub const StsError: i32 = -2;
pub const StsFilterOffsetErr: i32 = -31;
pub const StsFilterStructContentErr: i32 = -29;
pub const StsInplaceNotSupported: i32 = -203;
pub const StsInternal: i32 = -3;
pub const StsKernelStructContentErr: i32 = -30;
pub const StsNoConv: i32 = -7;
pub const StsNoMem: i32 = -4;
pub const StsNotImplemented: i32 = -213;
pub const StsNullPtr: i32 = -27;
pub const StsObjectNotFound: i32 = -204;
pub const StsOk: i32 = 0;
pub const StsOutOfRange: i32 = -211;
pub const StsParseError: i32 = -212;
pub const StsUnmatchedFormats: i32 = -205;
pub const StsUnmatchedSizes: i32 = -209;
pub const StsUnsupportedFormat: i32 = -210;
pub const StsVecLengthErr: i32 = -28;
pub const TEST_CUSTOM: i32 = 0;
pub const TEST_EQ: i32 = 1;
pub const TEST_GE: i32 = 5;
pub const TEST_GT: i32 = 6;
pub const TEST_LE: i32 = 3;
pub const TEST_LT: i32 = 4;
pub const TEST_NE: i32 = 2;
pub const TYPE_FUN: i32 = 0+3;
pub const TYPE_GENERAL: i32 = 0;
pub const TYPE_MARKER: i32 = 0+1;
pub const TYPE_WRAPPER: i32 = 0+2;
pub const TermCriteria_COUNT: i32 = 1;
pub const TermCriteria_EPS: i32 = 2;
pub const TermCriteria_MAX_ITER: i32 = 1;
pub const UMatData_ASYNC_CLEANUP: i32 = 128;
pub const UMatData_COPY_ON_MAP: i32 = 1;
pub const UMatData_DEVICE_COPY_OBSOLETE: i32 = 4;
pub const UMatData_DEVICE_MEM_MAPPED: i32 = 64;
pub const UMatData_HOST_COPY_OBSOLETE: i32 = 2;
pub const UMatData_TEMP_COPIED_UMAT: i32 = 24;
pub const UMatData_TEMP_UMAT: i32 = 8;
pub const UMatData_USER_ALLOCATED: i32 = 32;
pub const USAGE_ALLOCATE_DEVICE_MEMORY: i32 = 1 << 1;
pub const USAGE_ALLOCATE_HOST_MEMORY: i32 = 1 << 0;
pub const USAGE_ALLOCATE_SHARED_MEMORY: i32 = 1 << 2;
pub const USAGE_DEFAULT: i32 = 0;
pub const _InputArray_KIND_SHIFT: i32 = 16;
pub const __UMAT_USAGE_FLAGS_32BIT: i32 = 0x7fffffff;

pub type Vec8i = core::Vec8<i32>;
pub type Vec6d = core::Vec6<f64>;
pub type Vec6f = core::Vec6<f32>;
pub type Vec6i = core::Vec6<i32>;
pub type Vec4d = core::Vec4<f64>;
pub type Vec4f = core::Vec4<f32>;
pub type Vec4i = core::Vec4<i32>;
pub type Vec4w = core::Vec4<u16>;
pub type Vec4s = core::Vec4<i16>;
pub type Vec4b = core::Vec4<u8>;
pub type Vec3d = core::Vec3<f64>;
pub type Vec3f = core::Vec3<f32>;
pub type Vec3i = core::Vec3<i32>;
pub type Vec3w = core::Vec3<u16>;
pub type Vec3s = core::Vec3<i16>;
pub type Vec3b = core::Vec3<u8>;
pub type Vec2d = core::Vec2<f64>;
pub type Size2d = core::Size_<f64>;
pub type Point2d = core::Point_<f64>;
pub type Rect2d = core::Rect_<f64>;
pub type Vec2f = core::Vec2<f32>;
pub type Size2f = core::Size_<f32>;
pub type Point2f = core::Point_<f32>;
pub type Rect2f = core::Rect_<f32>;
pub type Size2l = core::Size_<i64>;
pub type Point2l = core::Point_<i64>;
pub type Vec2i = core::Vec2<i32>;
pub type Size2i = core::Size_<i32>;
pub type Point2i = core::Point_<i32>;
pub type Rect2i = core::Rect_<i32>;
pub type Size = core::Size_<i32>;
pub type Point = core::Point_<i32>;
pub type Rect = core::Rect_<i32>;
pub type Vec2w = core::Vec2<u16>;
pub type Vec2s = core::Vec2<i16>;
pub type Vec2b = core::Vec2<u8>;
pub type Scalar = core::Scalar_<f64>;
/// Class for matching keypoint descriptors
/// 
/// query descriptor index, train descriptor index, train image index, and distance between
/// descriptors.
#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct DMatch {
    pub query_idx: i32,
    pub train_idx: i32,
    pub img_idx: i32,
    pub distance: f32,
}

/// Data structure for salient point detectors.
/// 
/// The class instance stores a keypoint, i.e. a point feature found by one of many available keypoint
/// detectors, such as Harris corner detector, #FAST, %StarDetector, %SURF, %SIFT etc.
/// 
/// The keypoint is characterized by the 2D position, scale (proportional to the diameter of the
/// neighborhood that needs to be taken into account), orientation and some other parameters. The
/// keypoint neighborhood is then analyzed by another algorithm that builds a descriptor (usually
/// represented as a feature vector). The keypoints representing the same object in different images
/// can then be matched using %KDTree or another method.
#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct KeyPoint {
    pub pt: core::Point2f,
    pub size: f32,
    pub angle: f32,
    pub response: f32,
    pub octave: i32,
    pub class_id: i32,
}

/// struct returned by cv::moments
/// 
/// The spatial moments <span lang='latex'>\texttt{Moments::m}_{ji}</span> are computed as:
/// 
/// <div lang='latex'>\texttt{m} _{ji}= \sum _{x,y}  \left ( \texttt{array} (x,y)  \cdot x^j  \cdot y^i \right )</div>
/// 
/// The central moments <span lang='latex'>\texttt{Moments::mu}_{ji}</span> are computed as:
/// 
/// <div lang='latex'>\texttt{mu} _{ji}= \sum _{x,y}  \left ( \texttt{array} (x,y)  \cdot (x -  \bar{x} )^j  \cdot (y -  \bar{y} )^i \right )</div>
/// 
/// where <span lang='latex'>(\bar{x}, \bar{y})</span> is the mass center:
/// 
/// <div lang='latex'>\bar{x} = \frac{\texttt{m}_{10}}{\texttt{m}_{00}} , \; \bar{y} = \frac{\texttt{m}_{01}}{\texttt{m}_{00}}</div>
/// 
/// The normalized central moments <span lang='latex'>\texttt{Moments::nu}_{ij}</span> are computed as:
/// 
/// <div lang='latex'>\texttt{nu} _{ji}= \frac{\texttt{mu}_{ji}}{\texttt{m}_{00}^{(i+j)/2+1}} .</div>
/// 
/// 
/// Note:
/// <span lang='latex'>\texttt{mu}_{00}=\texttt{m}_{00}</span>, <span lang='latex'>\texttt{nu}_{00}=1</span>
/// <span lang='latex'>\texttt{nu}_{10}=\texttt{mu}_{10}=\texttt{mu}_{01}=\texttt{mu}_{10}=0</span> , hence the values are not
/// stored.
/// 
/// The moments of a contour are defined in the same way but computed using the Green's formula (see
/// <http://en.wikipedia.org/wiki/Green_theorem>). So, due to a limited raster resolution, the moments
/// computed for a contour are slightly different from the moments computed for the same rasterized
/// contour.
/// 
/// 
/// Note:
/// Since the contour moments are computed using Green formula, you may get seemingly odd results for
/// contours with self-intersections, e.g. a zero area (m00) for butterfly-shaped contours.
#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Moments {
    pub m00: f64,
    pub m10: f64,
    pub m01: f64,
    pub m20: f64,
    pub m11: f64,
    pub m02: f64,
    pub m30: f64,
    pub m21: f64,
    pub m12: f64,
    pub m03: f64,
    pub mu20: f64,
    pub mu11: f64,
    pub mu02: f64,
    pub mu30: f64,
    pub mu21: f64,
    pub mu12: f64,
    pub mu03: f64,
    pub nu20: f64,
    pub nu11: f64,
    pub nu02: f64,
    pub nu30: f64,
    pub nu21: f64,
    pub nu12: f64,
    pub nu03: f64,
}

/// proxy for hal::Cholesky
pub fn cholesky(a: &mut f64, astep: size_t, m: i32, b: &mut f64, bstep: size_t, n: i32) -> Result<bool> {
    unsafe { sys::cv_Cholesky_double_X_size_t_int_double_X_size_t_int(a, astep, m, b, bstep, n) }.into_result()
}

/// proxy for hal::Cholesky
pub fn cholesky_f32(a: &mut f32, astep: size_t, m: i32, b: &mut f32, bstep: size_t, n: i32) -> Result<bool> {
    unsafe { sys::cv_Cholesky_float_X_size_t_int_float_X_size_t_int(a, astep, m, b, bstep, n) }.into_result()
}

/// Performs a look-up table transform of an array.
/// 
/// The function LUT fills the output array with values from the look-up table. Indices of the entries
/// are taken from the input array. That is, the function processes each element of src as follows:
/// <div lang='latex'>\texttt{dst} (I)  \leftarrow \texttt{lut(src(I) + d)}</div>
/// where
/// <div lang='latex'>d =  \fork{0}{if \(\texttt{src}\) has depth \(\texttt{CV_8U}\)}{128}{if \(\texttt{src}\) has depth \(\texttt{CV_8S}\)}</div>
/// ## Parameters
/// * src: input array of 8-bit elements.
/// * lut: look-up table of 256 elements; in case of multi-channel input array, the table should
/// either have a single channel (in this case the same table is used for all channels) or the same
/// number of channels as in the input array.
/// * dst: output array of the same size and number of channels as src, and the same depth as lut.
/// ## See also
/// convertScaleAbs, Mat::convertTo
pub fn lut(src: &core::Mat, lut: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_LUT_Mat_Mat_Mat(src.as_raw_Mat(), lut.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// proxy for hal::LU
pub fn lu(a: &mut f64, astep: size_t, m: i32, b: &mut f64, bstep: size_t, n: i32) -> Result<i32> {
    unsafe { sys::cv_LU_double_X_size_t_int_double_X_size_t_int(a, astep, m, b, bstep, n) }.into_result()
}

/// proxy for hal::LU
pub fn lu_f32(a: &mut f32, astep: size_t, m: i32, b: &mut f32, bstep: size_t, n: i32) -> Result<i32> {
    unsafe { sys::cv_LU_float_X_size_t_int_float_X_size_t_int(a, astep, m, b, bstep, n) }.into_result()
}

/// Calculates the Mahalanobis distance between two vectors.
/// 
/// The function cv::Mahalanobis calculates and returns the weighted distance between two vectors:
/// <div lang='latex'>d( \texttt{vec1} , \texttt{vec2} )= \sqrt{\sum_{i,j}{\texttt{icovar(i,j)}\cdot(\texttt{vec1}(I)-\texttt{vec2}(I))\cdot(\texttt{vec1(j)}-\texttt{vec2(j)})} }</div>
/// The covariance matrix may be calculated using the #calcCovarMatrix function and then inverted using
/// the invert function (preferably using the #DECOMP_SVD method, as the most accurate).
/// ## Parameters
/// * v1: first 1D input vector.
/// * v2: second 1D input vector.
/// * icovar: inverse covariance matrix.
pub fn mahalanobis(v1: &core::Mat, v2: &core::Mat, icovar: &core::Mat) -> Result<f64> {
    unsafe { sys::cv_Mahalanobis_Mat_Mat_Mat(v1.as_raw_Mat(), v2.as_raw_Mat(), icovar.as_raw_Mat()) }.into_result()
}

/// wrap PCA::backProject
pub fn pca_back_project(data: &core::Mat, mean: &core::Mat, eigenvectors: &core::Mat, result: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_PCABackProject_Mat_Mat_Mat_Mat(data.as_raw_Mat(), mean.as_raw_Mat(), eigenvectors.as_raw_Mat(), result.as_raw_Mat()) }.into_result()
}

/// wrap PCA::operator() and add eigenvalues output parameter
pub fn pca_compute_values_variance(data: &core::Mat, mean: &mut core::Mat, eigenvectors: &mut core::Mat, eigenvalues: &mut core::Mat, retained_variance: f64) -> Result<()> {
    unsafe { sys::cv_PCACompute_Mat_Mat_Mat_Mat_double(data.as_raw_Mat(), mean.as_raw_Mat(), eigenvectors.as_raw_Mat(), eigenvalues.as_raw_Mat(), retained_variance) }.into_result()
}

/// wrap PCA::operator() and add eigenvalues output parameter
///
/// ## C++ default parameters
/// * max_components: 0
pub fn pca_compute_values(data: &core::Mat, mean: &mut core::Mat, eigenvectors: &mut core::Mat, eigenvalues: &mut core::Mat, max_components: i32) -> Result<()> {
    unsafe { sys::cv_PCACompute_Mat_Mat_Mat_Mat_int(data.as_raw_Mat(), mean.as_raw_Mat(), eigenvectors.as_raw_Mat(), eigenvalues.as_raw_Mat(), max_components) }.into_result()
}

/// wrap PCA::operator()
pub fn pca_compute_variance(data: &core::Mat, mean: &mut core::Mat, eigenvectors: &mut core::Mat, retained_variance: f64) -> Result<()> {
    unsafe { sys::cv_PCACompute_Mat_Mat_Mat_double(data.as_raw_Mat(), mean.as_raw_Mat(), eigenvectors.as_raw_Mat(), retained_variance) }.into_result()
}

/// wrap PCA::operator()
///
/// ## C++ default parameters
/// * max_components: 0
pub fn pca_compute(data: &core::Mat, mean: &mut core::Mat, eigenvectors: &mut core::Mat, max_components: i32) -> Result<()> {
    unsafe { sys::cv_PCACompute_Mat_Mat_Mat_int(data.as_raw_Mat(), mean.as_raw_Mat(), eigenvectors.as_raw_Mat(), max_components) }.into_result()
}

/// wrap PCA::project
pub fn pca_project(data: &core::Mat, mean: &core::Mat, eigenvectors: &core::Mat, result: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_PCAProject_Mat_Mat_Mat_Mat(data.as_raw_Mat(), mean.as_raw_Mat(), eigenvectors.as_raw_Mat(), result.as_raw_Mat()) }.into_result()
}

/// Computes the Peak Signal-to-Noise Ratio (PSNR) image quality metric.
/// 
/// This function calculates the Peak Signal-to-Noise Ratio (PSNR) image quality metric in decibels (dB), between two input arrays src1 and src2. Arrays must have depth CV_8U.
/// 
/// The PSNR is calculated as follows:
/// 
/// <div lang='latex'>
/// \texttt{PSNR} = 10 \cdot \log_{10}{\left( \frac{R^2}{MSE} \right) }
/// </div>
/// 
/// where R is the maximum integer value of depth CV_8U (255) and MSE is the mean squared error between the two arrays.
/// 
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size as src1.
pub fn psnr(src1: &core::Mat, src2: &core::Mat) -> Result<f64> {
    unsafe { sys::cv_PSNR_Mat_Mat(src1.as_raw_Mat(), src2.as_raw_Mat()) }.into_result()
}

/// wrap SVD::backSubst
pub fn sv_back_subst(w: &core::Mat, u: &core::Mat, vt: &core::Mat, rhs: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_SVBackSubst_Mat_Mat_Mat_Mat_Mat(w.as_raw_Mat(), u.as_raw_Mat(), vt.as_raw_Mat(), rhs.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// wrap SVD::compute
///
/// ## C++ default parameters
/// * flags: 0
pub fn sv_decomp(src: &core::Mat, w: &mut core::Mat, u: &mut core::Mat, vt: &mut core::Mat, flags: i32) -> Result<()> {
    unsafe { sys::cv_SVDecomp_Mat_Mat_Mat_Mat_int(src.as_raw_Mat(), w.as_raw_Mat(), u.as_raw_Mat(), vt.as_raw_Mat(), flags) }.into_result()
}

/// Calculates the per-element absolute difference between two arrays or between an array and a scalar.
/// 
/// The function cv::absdiff calculates:
///   Absolute difference between two arrays when they have the same
/// size and type:
/// <div lang='latex'>\texttt{dst}(I) =  \texttt{saturate} (| \texttt{src1}(I) -  \texttt{src2}(I)|)</div>
///   Absolute difference between an array and a scalar when the second
/// array is constructed from Scalar or has as many elements as the
/// number of channels in `src1`:
/// <div lang='latex'>\texttt{dst}(I) =  \texttt{saturate} (| \texttt{src1}(I) -  \texttt{src2} |)</div>
///   Absolute difference between a scalar and an array when the first
/// array is constructed from Scalar or has as many elements as the
/// number of channels in `src2`:
/// <div lang='latex'>\texttt{dst}(I) =  \texttt{saturate} (| \texttt{src1} -  \texttt{src2}(I) |)</div>
/// where I is a multi-dimensional index of array elements. In case of
/// multi-channel arrays, each channel is processed independently.
/// 
/// Note: Saturation is not applied when the arrays have the depth CV_32S.
/// You may even get a negative value in the case of overflow.
/// ## Parameters
/// * src1: first input array or a scalar.
/// * src2: second input array or a scalar.
/// * dst: output array that has the same size and type as input arrays.
/// ## See also
/// cv::abs(const Mat&)
pub fn absdiff(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_absdiff_Mat_Mat_Mat(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Calculates the weighted sum of two arrays.
/// 
/// The function addWeighted calculates the weighted sum of two arrays as follows:
/// <div lang='latex'>\texttt{dst} (I)= \texttt{saturate} ( \texttt{src1} (I)* \texttt{alpha} +  \texttt{src2} (I)* \texttt{beta} +  \texttt{gamma} )</div>
/// where I is a multi-dimensional index of array elements. In case of multi-channel arrays, each
/// channel is processed independently.
/// The function can be replaced with a matrix expression:
/// ```ignore{.cpp}
/// dst = src1*alpha + src2*beta + gamma;
/// ```
/// 
/// 
/// Note: Saturation is not applied when the output array has the depth CV_32S. You may even get
/// result of an incorrect sign in the case of overflow.
/// ## Parameters
/// * src1: first input array.
/// * alpha: weight of the first array elements.
/// * src2: second input array of the same size and channel number as src1.
/// * beta: weight of the second array elements.
/// * gamma: scalar added to each sum.
/// * dst: output array that has the same size and number of channels as the input arrays.
/// * dtype: optional depth of the output array; when both input arrays have the same depth, dtype
/// can be set to -1, which will be equivalent to src1.depth().
/// ## See also
/// add, subtract, scaleAdd, Mat::convertTo
///
/// ## C++ default parameters
/// * dtype: -1
pub fn add_weighted(src1: &core::Mat, alpha: f64, src2: &core::Mat, beta: f64, gamma: f64, dst: &mut core::Mat, dtype: i32) -> Result<()> {
    unsafe { sys::cv_addWeighted_Mat_double_Mat_double_double_Mat_int(src1.as_raw_Mat(), alpha, src2.as_raw_Mat(), beta, gamma, dst.as_raw_Mat(), dtype) }.into_result()
}

/// Calculates the per-element sum of two arrays or an array and a scalar.
/// 
/// The function add calculates:
/// - Sum of two arrays when both input arrays have the same size and the same number of channels:
/// <div lang='latex'>\texttt{dst}(I) =  \texttt{saturate} ( \texttt{src1}(I) +  \texttt{src2}(I)) \quad \texttt{if mask}(I) \ne0</div>
/// - Sum of an array and a scalar when src2 is constructed from Scalar or has the same number of
/// elements as `src1.channels()`:
/// <div lang='latex'>\texttt{dst}(I) =  \texttt{saturate} ( \texttt{src1}(I) +  \texttt{src2} ) \quad \texttt{if mask}(I) \ne0</div>
/// - Sum of a scalar and an array when src1 is constructed from Scalar or has the same number of
/// elements as `src2.channels()`:
/// <div lang='latex'>\texttt{dst}(I) =  \texttt{saturate} ( \texttt{src1} +  \texttt{src2}(I) ) \quad \texttt{if mask}(I) \ne0</div>
/// where `I` is a multi-dimensional index of array elements. In case of multi-channel arrays, each
/// channel is processed independently.
/// 
/// The first function in the list above can be replaced with matrix expressions:
/// ```ignore{.cpp}
/// dst = src1 + src2;
/// dst += src1; // equivalent to add(dst, src1, dst);
/// ```
/// 
/// The input arrays and the output array can all have the same or different depths. For example, you
/// can add a 16-bit unsigned array to a 8-bit signed array and store the sum as a 32-bit
/// floating-point array. Depth of the output array is determined by the dtype parameter. In the second
/// and third cases above, as well as in the first case, when src1.depth() == src2.depth(), dtype can
/// be set to the default -1. In this case, the output array will have the same depth as the input
/// array, be it src1, src2 or both.
/// 
/// Note: Saturation is not applied when the output array has the depth CV_32S. You may even get
/// result of an incorrect sign in the case of overflow.
/// ## Parameters
/// * src1: first input array or a scalar.
/// * src2: second input array or a scalar.
/// * dst: output array that has the same size and number of channels as the input array(s); the
/// depth is defined by dtype or src1/src2.
/// * mask: optional operation mask - 8-bit single channel array, that specifies elements of the
/// output array to be changed.
/// * dtype: optional depth of the output array (see the discussion below).
/// ## See also
/// subtract, addWeighted, scaleAdd, Mat::convertTo
///
/// ## C++ default parameters
/// * mask: noArray()
/// * dtype: -1
pub fn add(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat, mask: &core::Mat, dtype: i32) -> Result<()> {
    unsafe { sys::cv_add_Mat_Mat_Mat_Mat_int(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat(), mask.as_raw_Mat(), dtype) }.into_result()
}

/// Aligns a buffer size to the specified number of bytes.
/// 
/// The function returns the minimum number that is greater than or equal to sz and is divisible by n :
/// <div lang='latex'>\texttt{(sz + n-1) & -n}</div>
/// ## Parameters
/// * sz: Buffer size to align.
/// * n: Alignment size that must be a power of two.
pub fn align_size(sz: size_t, n: i32) -> Result<size_t> {
    unsafe { sys::cv_alignSize_size_t_int(sz, n) }.into_result()
}

/// naive nearest neighbor finder
/// 
/// see http://en.wikipedia.org/wiki/Nearest_neighbor_search
/// @todo document
///
/// ## C++ default parameters
/// * norm_type: NORM_L2
/// * k: 0
/// * mask: noArray()
/// * update: 0
/// * crosscheck: false
pub fn batch_distance(src1: &core::Mat, src2: &core::Mat, dist: &mut core::Mat, dtype: i32, nidx: &mut core::Mat, norm_type: i32, k: i32, mask: &core::Mat, update: i32, crosscheck: bool) -> Result<()> {
    unsafe { sys::cv_batchDistance_Mat_Mat_Mat_int_Mat_int_int_Mat_int_bool(src1.as_raw_Mat(), src2.as_raw_Mat(), dist.as_raw_Mat(), dtype, nidx.as_raw_Mat(), norm_type, k, mask.as_raw_Mat(), update, crosscheck) }.into_result()
}

/// computes bitwise conjunction of the two arrays (dst = src1 & src2)
/// Calculates the per-element bit-wise conjunction of two arrays or an
/// array and a scalar.
/// 
/// The function cv::bitwise_and calculates the per-element bit-wise logical conjunction for:
///   Two arrays when src1 and src2 have the same size:
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{src1} (I)  \wedge \texttt{src2} (I) \quad \texttt{if mask} (I) \ne0</div>
///   An array and a scalar when src2 is constructed from Scalar or has
/// the same number of elements as `src1.channels()`:
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{src1} (I)  \wedge \texttt{src2} \quad \texttt{if mask} (I) \ne0</div>
///   A scalar and an array when src1 is constructed from Scalar or has
/// the same number of elements as `src2.channels()`:
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{src1}  \wedge \texttt{src2} (I) \quad \texttt{if mask} (I) \ne0</div>
/// In case of floating-point arrays, their machine-specific bit
/// representations (usually IEEE754-compliant) are used for the operation.
/// In case of multi-channel arrays, each channel is processed
/// independently. In the second and third cases above, the scalar is first
/// converted to the array type.
/// ## Parameters
/// * src1: first input array or a scalar.
/// * src2: second input array or a scalar.
/// * dst: output array that has the same size and type as the input
/// arrays.
/// * mask: optional operation mask, 8-bit single channel array, that
/// specifies elements of the output array to be changed.
///
/// ## C++ default parameters
/// * mask: noArray()
pub fn bitwise_and(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_bitwise_and_Mat_Mat_Mat_Mat(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
}

/// Inverts every bit of an array.
/// 
/// The function cv::bitwise_not calculates per-element bit-wise inversion of the input
/// array:
/// <div lang='latex'>\texttt{dst} (I) =  \neg \texttt{src} (I)</div>
/// In case of a floating-point input array, its machine-specific bit
/// representation (usually IEEE754-compliant) is used for the operation. In
/// case of multi-channel arrays, each channel is processed independently.
/// ## Parameters
/// * src: input array.
/// * dst: output array that has the same size and type as the input
/// array.
/// * mask: optional operation mask, 8-bit single channel array, that
/// specifies elements of the output array to be changed.
///
/// ## C++ default parameters
/// * mask: noArray()
pub fn bitwise_not(src: &core::Mat, dst: &mut core::Mat, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_bitwise_not_Mat_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
}

/// Calculates the per-element bit-wise disjunction of two arrays or an
/// array and a scalar.
/// 
/// The function cv::bitwise_or calculates the per-element bit-wise logical disjunction for:
///   Two arrays when src1 and src2 have the same size:
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{src1} (I)  \vee \texttt{src2} (I) \quad \texttt{if mask} (I) \ne0</div>
///   An array and a scalar when src2 is constructed from Scalar or has
/// the same number of elements as `src1.channels()`:
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{src1} (I)  \vee \texttt{src2} \quad \texttt{if mask} (I) \ne0</div>
///   A scalar and an array when src1 is constructed from Scalar or has
/// the same number of elements as `src2.channels()`:
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{src1}  \vee \texttt{src2} (I) \quad \texttt{if mask} (I) \ne0</div>
/// In case of floating-point arrays, their machine-specific bit
/// representations (usually IEEE754-compliant) are used for the operation.
/// In case of multi-channel arrays, each channel is processed
/// independently. In the second and third cases above, the scalar is first
/// converted to the array type.
/// ## Parameters
/// * src1: first input array or a scalar.
/// * src2: second input array or a scalar.
/// * dst: output array that has the same size and type as the input
/// arrays.
/// * mask: optional operation mask, 8-bit single channel array, that
/// specifies elements of the output array to be changed.
///
/// ## C++ default parameters
/// * mask: noArray()
pub fn bitwise_or(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_bitwise_or_Mat_Mat_Mat_Mat(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
}

/// Calculates the per-element bit-wise "exclusive or" operation on two
/// arrays or an array and a scalar.
/// 
/// The function cv::bitwise_xor calculates the per-element bit-wise logical "exclusive-or"
/// operation for:
///   Two arrays when src1 and src2 have the same size:
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{src1} (I)  \oplus \texttt{src2} (I) \quad \texttt{if mask} (I) \ne0</div>
///   An array and a scalar when src2 is constructed from Scalar or has
/// the same number of elements as `src1.channels()`:
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{src1} (I)  \oplus \texttt{src2} \quad \texttt{if mask} (I) \ne0</div>
///   A scalar and an array when src1 is constructed from Scalar or has
/// the same number of elements as `src2.channels()`:
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{src1}  \oplus \texttt{src2} (I) \quad \texttt{if mask} (I) \ne0</div>
/// In case of floating-point arrays, their machine-specific bit
/// representations (usually IEEE754-compliant) are used for the operation.
/// In case of multi-channel arrays, each channel is processed
/// independently. In the 2nd and 3rd cases above, the scalar is first
/// converted to the array type.
/// ## Parameters
/// * src1: first input array or a scalar.
/// * src2: second input array or a scalar.
/// * dst: output array that has the same size and type as the input
/// arrays.
/// * mask: optional operation mask, 8-bit single channel array, that
/// specifies elements of the output array to be changed.
///
/// ## C++ default parameters
/// * mask: noArray()
pub fn bitwise_xor(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_bitwise_xor_Mat_Mat_Mat_Mat(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
}

/// Computes the source location of an extrapolated pixel.
/// 
/// The function computes and returns the coordinate of a donor pixel corresponding to the specified
/// extrapolated pixel when using the specified extrapolation border mode. For example, if you use
/// cv::BORDER_WRAP mode in the horizontal direction, cv::BORDER_REFLECT_101 in the vertical direction and
/// want to compute value of the "virtual" pixel Point(-5, 100) in a floating-point image img , it
/// looks like:
/// ```ignore{.cpp}
/// float val = img.at<float>(borderInterpolate(100, img.rows, cv::BORDER_REFLECT_101),
/// borderInterpolate(-5, img.cols, cv::BORDER_WRAP));
/// ```
/// 
/// Normally, the function is not called directly. It is used inside filtering functions and also in
/// copyMakeBorder.
/// ## Parameters
/// * p: 0-based coordinate of the extrapolated pixel along one of the axes, likely \<0 or \>= len
/// * len: Length of the array along the corresponding axis.
/// * borderType: Border type, one of the #BorderTypes, except for #BORDER_TRANSPARENT and
/// #BORDER_ISOLATED . When borderType==#BORDER_CONSTANT , the function always returns -1, regardless
/// of p and len.
/// 
/// ## See also
/// copyMakeBorder
pub fn border_interpolate(p: i32, len: i32, border_type: i32) -> Result<i32> {
    unsafe { sys::cv_borderInterpolate_int_int_int(p, len, border_type) }.into_result()
}

/// Calculates the covariance matrix of a set of vectors.
/// 
/// The function cv::calcCovarMatrix calculates the covariance matrix and, optionally, the mean vector of
/// the set of input vectors.
/// ## Parameters
/// * samples: samples stored as separate matrices
/// * nsamples: number of samples
/// * covar: output covariance matrix of the type ctype and square size.
/// * mean: input or output (depending on the flags) array as the average value of the input vectors.
/// * flags: operation flags as a combination of #CovarFlags
/// * ctype: type of the matrixl; it equals 'CV_64F' by default.
/// ## See also
/// PCA, mulTransposed, Mahalanobis
/// @todo InputArrayOfArrays
/// 
/// ## Overloaded parameters
/// 
/// 
/// Note: use #COVAR_ROWS or #COVAR_COLS flag
/// * samples: samples stored as rows/columns of a single matrix.
/// * covar: output covariance matrix of the type ctype and square size.
/// * mean: input or output (depending on the flags) array as the average value of the input vectors.
/// * flags: operation flags as a combination of #CovarFlags
/// * ctype: type of the matrixl; it equals 'CV_64F' by default.
///
/// ## C++ default parameters
/// * ctype: CV_64F
pub fn calc_covar_matrix(samples: &core::Mat, covar: &mut core::Mat, mean: &mut core::Mat, flags: i32, ctype: i32) -> Result<()> {
    unsafe { sys::cv_calcCovarMatrix_Mat_Mat_Mat_int_int(samples.as_raw_Mat(), covar.as_raw_Mat(), mean.as_raw_Mat(), flags, ctype) }.into_result()
}

/// Calculates the magnitude and angle of 2D vectors.
/// 
/// The function cv::cartToPolar calculates either the magnitude, angle, or both
/// for every 2D vector (x(I),y(I)):
/// <div lang='latex'>\begin{array}{l} \texttt{magnitude} (I)= \sqrt{\texttt{x}(I)^2+\texttt{y}(I)^2} , \\ \texttt{angle} (I)= \texttt{atan2} ( \texttt{y} (I), \texttt{x} (I))[ \cdot180 / \pi ] \end{array}</div>
/// 
/// The angles are calculated with accuracy about 0.3 degrees. For the point
/// (0,0), the angle is set to 0.
/// ## Parameters
/// * x: array of x-coordinates; this must be a single-precision or
/// double-precision floating-point array.
/// * y: array of y-coordinates, that must have the same size and same type as x.
/// * magnitude: output array of magnitudes of the same size and type as x.
/// * angle: output array of angles that has the same size and type as
/// x; the angles are measured in radians (from 0 to 2\*Pi) or in degrees (0 to 360 degrees).
/// * angleInDegrees: a flag, indicating whether the angles are measured
/// in radians (which is by default), or in degrees.
/// ## See also
/// Sobel, Scharr
///
/// ## C++ default parameters
/// * angle_in_degrees: false
pub fn cart_to_polar(x: &core::Mat, y: &core::Mat, magnitude: &mut core::Mat, angle: &mut core::Mat, angle_in_degrees: bool) -> Result<()> {
    unsafe { sys::cv_cartToPolar_Mat_Mat_Mat_Mat_bool(x.as_raw_Mat(), y.as_raw_Mat(), magnitude.as_raw_Mat(), angle.as_raw_Mat(), angle_in_degrees) }.into_result()
}

/// Returns true if the specified feature is supported by the host hardware.
/// 
/// The function returns true if the host hardware supports the specified feature. When user calls
/// setUseOptimized(false), the subsequent calls to checkHardwareSupport() will return false until
/// setUseOptimized(true) is called. This way user can dynamically switch on and off the optimized code
/// in OpenCV.
/// ## Parameters
/// * feature: The feature of interest, one of cv::CpuFeatures
pub fn check_hardware_support(feature: i32) -> Result<bool> {
    unsafe { sys::cv_checkHardwareSupport_int(feature) }.into_result()
}

/// Checks every element of an input array for invalid values.
/// 
/// The function cv::checkRange checks that every array element is neither NaN nor infinite. When minVal \>
/// -DBL_MAX and maxVal \< DBL_MAX, the function also checks that each value is between minVal and
/// maxVal. In case of multi-channel arrays, each channel is processed independently. If some values
/// are out of range, position of the first outlier is stored in pos (when pos != NULL). Then, the
/// function either returns false (when quiet=true) or throws an exception.
/// ## Parameters
/// * a: input array.
/// * quiet: a flag, indicating whether the functions quietly return false when the array elements
/// are out of range or they throw an exception.
/// * pos: optional output parameter, when not NULL, must be a pointer to array of src.dims
/// elements.
/// * minVal: inclusive lower boundary of valid values range.
/// * maxVal: exclusive upper boundary of valid values range.
///
/// ## C++ default parameters
/// * quiet: true
/// * pos: 0
/// * min_val: -DBL_MAX
/// * max_val: DBL_MAX
pub fn check_range(a: &core::Mat, quiet: bool, pos: &mut core::Point, min_val: f64, max_val: f64) -> Result<bool> {
    unsafe { sys::cv_checkRange_Mat_bool_Point_X_double_double(a.as_raw_Mat(), quiet, pos, min_val, max_val) }.into_result()
}

/// Performs the per-element comparison of two arrays or an array and scalar value.
/// 
/// The function compares:
///   Elements of two arrays when src1 and src2 have the same size:
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{src1} (I)  \,\texttt{cmpop}\, \texttt{src2} (I)</div>
///   Elements of src1 with a scalar src2 when src2 is constructed from
/// Scalar or has a single element:
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{src1}(I) \,\texttt{cmpop}\,  \texttt{src2}</div>
///   src1 with elements of src2 when src1 is constructed from Scalar or
/// has a single element:
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{src1}  \,\texttt{cmpop}\, \texttt{src2} (I)</div>
/// When the comparison result is true, the corresponding element of output
/// array is set to 255. The comparison operations can be replaced with the
/// equivalent matrix expressions:
/// ```ignore{.cpp}
/// Mat dst1 = src1 >= src2;
/// Mat dst2 = src1 < 8;
/// ...
/// ```
/// 
/// ## Parameters
/// * src1: first input array or a scalar; when it is an array, it must have a single channel.
/// * src2: second input array or a scalar; when it is an array, it must have a single channel.
/// * dst: output array of type ref CV_8U that has the same size and the same number of channels as
/// the input arrays.
/// * cmpop: a flag, that specifies correspondence between the arrays (cv::CmpTypes)
/// ## See also
/// checkRange, min, max, threshold
pub fn compare(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat, cmpop: i32) -> Result<()> {
    unsafe { sys::cv_compare_Mat_Mat_Mat_int(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat(), cmpop) }.into_result()
}

/// Copies the lower or the upper half of a square matrix to its another half.
/// 
/// The function cv::completeSymm copies the lower or the upper half of a square matrix to
/// its another half. The matrix diagonal remains unchanged:
/// - <span lang='latex'>\texttt{m}_{ij}=\texttt{m}_{ji}</span> for <span lang='latex'>i > j</span> if
/// lowerToUpper=false
/// - <span lang='latex'>\texttt{m}_{ij}=\texttt{m}_{ji}</span> for <span lang='latex'>i < j</span> if
/// lowerToUpper=true
/// 
/// ## Parameters
/// * m: input-output floating-point square matrix.
/// * lowerToUpper: operation flag; if true, the lower half is copied to
/// the upper half. Otherwise, the upper half is copied to the lower half.
/// ## See also
/// flip, transpose
///
/// ## C++ default parameters
/// * lower_to_upper: false
pub fn complete_symm(m: &mut core::Mat, lower_to_upper: bool) -> Result<()> {
    unsafe { sys::cv_completeSymm_Mat_bool(m.as_raw_Mat(), lower_to_upper) }.into_result()
}

/// Converts an array to half precision floating number.
/// 
/// This function converts FP32 (single precision floating point) from/to FP16 (half precision floating point). CV_16S format is used to represent FP16 data.
/// There are two use modes (src -> dst): CV_32F -> CV_16S and CV_16S -> CV_32F. The input array has to have type of CV_32F or
/// CV_16S to represent the bit depth. If the input array is neither of them, the function will raise an error.
/// The format of half precision floating point is defined in IEEE 754-2008.
/// 
/// ## Parameters
/// * src: input array.
/// * dst: output array.
pub fn convert_fp16(src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_convertFp16_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Scales, calculates absolute values, and converts the result to 8-bit.
/// 
/// On each element of the input array, the function convertScaleAbs
/// performs three operations sequentially: scaling, taking an absolute
/// value, conversion to an unsigned 8-bit type:
/// <div lang='latex'>\texttt{dst} (I)= \texttt{saturate\_cast<uchar>} (| \texttt{src} (I)* \texttt{alpha} +  \texttt{beta} |)</div>
/// In case of multi-channel arrays, the function processes each channel
/// independently. When the output is not 8-bit, the operation can be
/// emulated by calling the Mat::convertTo method (or by using matrix
/// expressions) and then by calculating an absolute value of the result.
/// For example:
/// ```ignore{.cpp}
/// Mat_<float> A(30,30);
/// randu(A, Scalar(-100), Scalar(100));
/// Mat_<float> B = A*5 + 3;
/// B = abs(B);
/// // Mat_<float> B = abs(A*5+3) will also do the job,
/// // but it will allocate a temporary matrix
/// ```
/// 
/// ## Parameters
/// * src: input array.
/// * dst: output array.
/// * alpha: optional scale factor.
/// * beta: optional delta added to the scaled values.
/// ## See also
/// Mat::convertTo, cv::abs(const Mat&)
///
/// ## C++ default parameters
/// * alpha: 1
/// * beta: 0
pub fn convert_scale_abs(src: &core::Mat, dst: &mut core::Mat, alpha: f64, beta: f64) -> Result<()> {
    unsafe { sys::cv_convertScaleAbs_Mat_Mat_double_double(src.as_raw_Mat(), dst.as_raw_Mat(), alpha, beta) }.into_result()
}

/// Forms a border around an image.
/// 
/// The function copies the source image into the middle of the destination image. The areas to the
/// left, to the right, above and below the copied source image will be filled with extrapolated
/// pixels. This is not what filtering functions based on it do (they extrapolate pixels on-fly), but
/// what other more complex functions, including your own, may do to simplify image boundary handling.
/// 
/// The function supports the mode when src is already in the middle of dst . In this case, the
/// function does not copy src itself but simply constructs the border, for example:
/// 
/// ```ignore{.cpp}
/// // let border be the same in all directions
/// int border=2;
/// // constructs a larger image to fit both the image and the border
/// Mat gray_buf(rgb.rows + border*2, rgb.cols + border*2, rgb.depth());
/// // select the middle part of it w/o copying data
/// Mat gray(gray_canvas, Rect(border, border, rgb.cols, rgb.rows));
/// // convert image from RGB to grayscale
/// cvtColor(rgb, gray, COLOR_RGB2GRAY);
/// // form a border in-place
/// copyMakeBorder(gray, gray_buf, border, border,
/// border, border, BORDER_REPLICATE);
/// // now do some custom filtering ...
/// ...
/// ```
/// 
/// 
/// Note: When the source image is a part (ROI) of a bigger image, the function will try to use the
/// pixels outside of the ROI to form a border. To disable this feature and always do extrapolation, as
/// if src was not a ROI, use borderType | #BORDER_ISOLATED.
/// 
/// ## Parameters
/// * src: Source image.
/// * dst: Destination image of the same type as src and the size Size(src.cols+left+right,
/// src.rows+top+bottom) .
/// * top: 
/// * bottom: 
/// * left: 
/// * right: Parameter specifying how many pixels in each direction from the source image rectangle
/// to extrapolate. For example, top=1, bottom=1, left=1, right=1 mean that 1 pixel-wide border needs
/// to be built.
/// * borderType: Border type. See borderInterpolate for details.
/// * value: Border value if borderType==BORDER_CONSTANT .
/// 
/// ## See also
/// borderInterpolate
///
/// ## C++ default parameters
/// * value: Scalar()
pub fn copy_make_border(src: &core::Mat, dst: &mut core::Mat, top: i32, bottom: i32, left: i32, right: i32, border_type: i32, value: core::Scalar) -> Result<()> {
    unsafe { sys::cv_copyMakeBorder_Mat_Mat_int_int_int_int_int_Scalar(src.as_raw_Mat(), dst.as_raw_Mat(), top, bottom, left, right, border_type, value) }.into_result()
}

/// Counts non-zero array elements.
/// 
/// The function returns the number of non-zero elements in src :
/// <div lang='latex'>\sum _{I: \; \texttt{src} (I) \ne0 } 1</div>
/// ## Parameters
/// * src: single-channel array.
/// ## See also
/// mean, meanStdDev, norm, minMaxLoc, calcCovarMatrix
pub fn count_non_zero(src: &core::Mat) -> Result<i32> {
    unsafe { sys::cv_countNonZero_Mat(src.as_raw_Mat()) }.into_result()
}

/// Computes the cube root of an argument.
/// 
/// The function cubeRoot computes <span lang='latex'>\sqrt[3]{\texttt{val}}</span>. Negative arguments are handled correctly.
/// NaN and Inf are not handled. The accuracy approaches the maximum possible accuracy for
/// single-precision data.
/// ## Parameters
/// * val: A function argument.
pub fn cube_root(val: f32) -> Result<f32> {
    unsafe { sys::cv_cubeRoot_float(val) }.into_result()
}

pub fn cv_abs(x: i8) -> Result<i32> {
    unsafe { sys::cv_cv_abs_schar(x) }.into_result()
}

pub fn cv_abs_1(x: u16) -> Result<i32> {
    unsafe { sys::cv_cv_abs_ushort(x) }.into_result()
}

/// Performs a forward or inverse discrete Cosine transform of 1D or 2D array.
/// 
/// The function cv::dct performs a forward or inverse discrete Cosine transform (DCT) of a 1D or 2D
/// floating-point array:
/// *   Forward Cosine transform of a 1D vector of N elements:
/// <div lang='latex'>Y = C^{(N)}  \cdot X</div>
/// where
/// <div lang='latex'>C^{(N)}_{jk}= \sqrt{\alpha_j/N} \cos \left ( \frac{\pi(2k+1)j}{2N} \right )</div>
/// and
/// <span lang='latex'>\alpha_0=1</span>, <span lang='latex'>\alpha_j=2</span> for *j \> 0*.
/// *   Inverse Cosine transform of a 1D vector of N elements:
/// <div lang='latex'>X =  \left (C^{(N)} \right )^{-1}  \cdot Y =  \left (C^{(N)} \right )^T  \cdot Y</div>
/// (since <span lang='latex'>C^{(N)}</span> is an orthogonal matrix, <span lang='latex'>C^{(N)} \cdot \left(C^{(N)}\right)^T = I</span> )
/// *   Forward 2D Cosine transform of M x N matrix:
/// <div lang='latex'>Y = C^{(N)}  \cdot X  \cdot \left (C^{(N)} \right )^T</div>
/// *   Inverse 2D Cosine transform of M x N matrix:
/// <div lang='latex'>X =  \left (C^{(N)} \right )^T  \cdot X  \cdot C^{(N)}</div>
/// 
/// The function chooses the mode of operation by looking at the flags and size of the input array:
/// *   If (flags & #DCT_INVERSE) == 0 , the function does a forward 1D or 2D transform. Otherwise, it
/// is an inverse 1D or 2D transform.
/// *   If (flags & #DCT_ROWS) != 0 , the function performs a 1D transform of each row.
/// *   If the array is a single column or a single row, the function performs a 1D transform.
/// *   If none of the above is true, the function performs a 2D transform.
/// 
/// 
/// Note: Currently dct supports even-size arrays (2, 4, 6 ...). For data analysis and approximation, you
/// can pad the array when necessary.
/// Also, the function performance depends very much, and not monotonically, on the array size (see
/// getOptimalDFTSize ). In the current implementation DCT of a vector of size N is calculated via DFT
/// of a vector of size N/2 . Thus, the optimal DCT size N1 \>= N can be calculated as:
/// ```ignore
/// size_t getOptimalDCTSize(size_t N) { return 2*getOptimalDFTSize((N+1)/2); }
/// N1 = getOptimalDCTSize(N);
/// ```
/// 
/// ## Parameters
/// * src: input floating-point array.
/// * dst: output array of the same size and type as src .
/// * flags: transformation flags as a combination of cv::DftFlags (DCT_*)
/// ## See also
/// dft , getOptimalDFTSize , idct
///
/// ## C++ default parameters
/// * flags: 0
pub fn dct(src: &core::Mat, dst: &mut core::Mat, flags: i32) -> Result<()> {
    unsafe { sys::cv_dct_Mat_Mat_int(src.as_raw_Mat(), dst.as_raw_Mat(), flags) }.into_result()
}

/// Returns string of cv::Mat depth value: CV_8U -> "CV_8U" or "<invalid depth>"
pub fn depth_to_string(depth: i32) -> Result<String> {
    unsafe { sys::cv_depthToString_int(depth) }.into_result().map(crate::templ::receive_string)
}

/// Returns string of cv::Mat depth value: CV_8U -> "CV_8U" or NULL
pub fn depth_to_string_(depth: i32) -> Result<String> {
    unsafe { sys::cv_detail_depthToString__int(depth) }.into_result().map(crate::templ::receive_string)
}

/// Returns string of cv::Mat depth value: CV_8UC3 -> "CV_8UC3" or cv::String()
pub fn type_to_string_(_type: i32) -> Result<String> {
    unsafe { sys::cv_detail_typeToString__int(_type) }.into_result().map(crate::templ::receive_string)
}

/// Returns the determinant of a square floating-point matrix.
/// 
/// The function cv::determinant calculates and returns the determinant of the
/// specified matrix. For small matrices ( mtx.cols=mtx.rows\<=3 ), the
/// direct method is used. For larger matrices, the function uses LU
/// factorization with partial pivoting.
/// 
/// For symmetric positively-determined matrices, it is also possible to use
/// eigen decomposition to calculate the determinant.
/// ## Parameters
/// * mtx: input matrix that must have CV_32FC1 or CV_64FC1 type and
/// square size.
/// ## See also
/// trace, invert, solve, eigen, @ref MatrixExpressions
pub fn determinant(mtx: &core::Mat) -> Result<f64> {
    unsafe { sys::cv_determinant_Mat(mtx.as_raw_Mat()) }.into_result()
}

/// Performs a forward or inverse Discrete Fourier transform of a 1D or 2D floating-point array.
/// 
/// The function cv::dft performs one of the following:
/// *   Forward the Fourier transform of a 1D vector of N elements:
/// <div lang='latex'>Y = F^{(N)}  \cdot X,</div>
/// where <span lang='latex'>F^{(N)}_{jk}=\exp(-2\pi i j k/N)</span> and <span lang='latex'>i=\sqrt{-1}</span>
/// *   Inverse the Fourier transform of a 1D vector of N elements:
/// <div lang='latex'>\begin{array}{l} X'=  \left (F^{(N)} \right )^{-1}  \cdot Y =  \left (F^{(N)} \right )^*  \cdot y  \\ X = (1/N)  \cdot X, \end{array}</div>
/// where <span lang='latex'>F^*=\left(\textrm{Re}(F^{(N)})-\textrm{Im}(F^{(N)})\right)^T</span>
/// *   Forward the 2D Fourier transform of a M x N matrix:
/// <div lang='latex'>Y = F^{(M)}  \cdot X  \cdot F^{(N)}</div>
/// *   Inverse the 2D Fourier transform of a M x N matrix:
/// <div lang='latex'>\begin{array}{l} X'=  \left (F^{(M)} \right )^*  \cdot Y  \cdot \left (F^{(N)} \right )^* \\ X =  \frac{1}{M \cdot N} \cdot X' \end{array}</div>
/// 
/// In case of real (single-channel) data, the output spectrum of the forward Fourier transform or input
/// spectrum of the inverse Fourier transform can be represented in a packed format called *CCS*
/// (complex-conjugate-symmetrical). It was borrowed from IPL (Intel\* Image Processing Library). Here
/// is how 2D *CCS* spectrum looks:
/// <div lang='latex'>\begin{bmatrix} Re Y_{0,0} & Re Y_{0,1} & Im Y_{0,1} & Re Y_{0,2} & Im Y_{0,2} &  \cdots & Re Y_{0,N/2-1} & Im Y_{0,N/2-1} & Re Y_{0,N/2}  \\ Re Y_{1,0} & Re Y_{1,1} & Im Y_{1,1} & Re Y_{1,2} & Im Y_{1,2} &  \cdots & Re Y_{1,N/2-1} & Im Y_{1,N/2-1} & Re Y_{1,N/2}  \\ Im Y_{1,0} & Re Y_{2,1} & Im Y_{2,1} & Re Y_{2,2} & Im Y_{2,2} &  \cdots & Re Y_{2,N/2-1} & Im Y_{2,N/2-1} & Im Y_{1,N/2}  \\ \hdotsfor{9} \\ Re Y_{M/2-1,0} &  Re Y_{M-3,1}  & Im Y_{M-3,1} &  \hdotsfor{3} & Re Y_{M-3,N/2-1} & Im Y_{M-3,N/2-1}& Re Y_{M/2-1,N/2}  \\ Im Y_{M/2-1,0} &  Re Y_{M-2,1}  & Im Y_{M-2,1} &  \hdotsfor{3} & Re Y_{M-2,N/2-1} & Im Y_{M-2,N/2-1}& Im Y_{M/2-1,N/2}  \\ Re Y_{M/2,0}  &  Re Y_{M-1,1} &  Im Y_{M-1,1} &  \hdotsfor{3} & Re Y_{M-1,N/2-1} & Im Y_{M-1,N/2-1}& Re Y_{M/2,N/2} \end{bmatrix}</div>
/// 
/// In case of 1D transform of a real vector, the output looks like the first row of the matrix above.
/// 
/// So, the function chooses an operation mode depending on the flags and size of the input array:
/// *   If #DFT_ROWS is set or the input array has a single row or single column, the function
/// performs a 1D forward or inverse transform of each row of a matrix when #DFT_ROWS is set.
/// Otherwise, it performs a 2D transform.
/// *   If the input array is real and #DFT_INVERSE is not set, the function performs a forward 1D or
/// 2D transform:
/// *   When #DFT_COMPLEX_OUTPUT is set, the output is a complex matrix of the same size as
/// input.
/// *   When #DFT_COMPLEX_OUTPUT is not set, the output is a real matrix of the same size as
/// input. In case of 2D transform, it uses the packed format as shown above. In case of a
/// single 1D transform, it looks like the first row of the matrix above. In case of
/// multiple 1D transforms (when using the #DFT_ROWS flag), each row of the output matrix
/// looks like the first row of the matrix above.
/// *   If the input array is complex and either #DFT_INVERSE or #DFT_REAL_OUTPUT are not set, the
/// output is a complex array of the same size as input. The function performs a forward or
/// inverse 1D or 2D transform of the whole input array or each row of the input array
/// independently, depending on the flags DFT_INVERSE and DFT_ROWS.
/// *   When #DFT_INVERSE is set and the input array is real, or it is complex but #DFT_REAL_OUTPUT
/// is set, the output is a real array of the same size as input. The function performs a 1D or 2D
/// inverse transformation of the whole input array or each individual row, depending on the flags
/// #DFT_INVERSE and #DFT_ROWS.
/// 
/// If #DFT_SCALE is set, the scaling is done after the transformation.
/// 
/// Unlike dct , the function supports arrays of arbitrary size. But only those arrays are processed
/// efficiently, whose sizes can be factorized in a product of small prime numbers (2, 3, and 5 in the
/// current implementation). Such an efficient DFT size can be calculated using the getOptimalDFTSize
/// method.
/// 
/// The sample below illustrates how to calculate a DFT-based convolution of two 2D real arrays:
/// ```ignore
/// void convolveDFT(InputArray A, InputArray B, OutputArray C)
/// {
/// // reallocate the output array if needed
/// C.create(abs(A.rows - B.rows)+1, abs(A.cols - B.cols)+1, A.type());
/// Size dftSize;
/// // calculate the size of DFT transform
/// dftSize.width = getOptimalDFTSize(A.cols + B.cols - 1);
/// dftSize.height = getOptimalDFTSize(A.rows + B.rows - 1);
/// 
/// // allocate temporary buffers and initialize them with 0's
/// Mat tempA(dftSize, A.type(), Scalar::all(0));
/// Mat tempB(dftSize, B.type(), Scalar::all(0));
/// 
/// // copy A and B to the top-left corners of tempA and tempB, respectively
/// Mat roiA(tempA, Rect(0,0,A.cols,A.rows));
/// A.copyTo(roiA);
/// Mat roiB(tempB, Rect(0,0,B.cols,B.rows));
/// B.copyTo(roiB);
/// 
/// // now transform the padded A & B in-place;
/// // use "nonzeroRows" hint for faster processing
/// dft(tempA, tempA, 0, A.rows);
/// dft(tempB, tempB, 0, B.rows);
/// 
/// // multiply the spectrums;
/// // the function handles packed spectrum representations well
/// mulSpectrums(tempA, tempB, tempA);
/// 
/// // transform the product back from the frequency domain.
/// // Even though all the result rows will be non-zero,
/// // you need only the first C.rows of them, and thus you
/// // pass nonzeroRows == C.rows
/// dft(tempA, tempA, DFT_INVERSE + DFT_SCALE, C.rows);
/// 
/// // now copy the result back to C.
/// tempA(Rect(0, 0, C.cols, C.rows)).copyTo(C);
/// 
/// // all the temporary buffers will be deallocated automatically
/// }
/// ```
/// 
/// To optimize this sample, consider the following approaches:
/// *   Since nonzeroRows != 0 is passed to the forward transform calls and since A and B are copied to
/// the top-left corners of tempA and tempB, respectively, it is not necessary to clear the whole
/// tempA and tempB. It is only necessary to clear the tempA.cols - A.cols ( tempB.cols - B.cols)
/// rightmost columns of the matrices.
/// *   This DFT-based convolution does not have to be applied to the whole big arrays, especially if B
/// is significantly smaller than A or vice versa. Instead, you can calculate convolution by parts.
/// To do this, you need to split the output array C into multiple tiles. For each tile, estimate
/// which parts of A and B are required to calculate convolution in this tile. If the tiles in C are
/// too small, the speed will decrease a lot because of repeated work. In the ultimate case, when
/// each tile in C is a single pixel, the algorithm becomes equivalent to the naive convolution
/// algorithm. If the tiles are too big, the temporary arrays tempA and tempB become too big and
/// there is also a slowdown because of bad cache locality. So, there is an optimal tile size
/// somewhere in the middle.
/// *   If different tiles in C can be calculated in parallel and, thus, the convolution is done by
/// parts, the loop can be threaded.
/// 
/// All of the above improvements have been implemented in #matchTemplate and #filter2D . Therefore, by
/// using them, you can get the performance even better than with the above theoretically optimal
/// implementation. Though, those two functions actually calculate cross-correlation, not convolution,
/// so you need to "flip" the second convolution operand B vertically and horizontally using flip .
/// 
/// Note:
/// *   An example using the discrete fourier transform can be found at
/// opencv_source_code/samples/cpp/dft.cpp
/// *   (Python) An example using the dft functionality to perform Wiener deconvolution can be found
/// at opencv_source/samples/python/deconvolution.py
/// *   (Python) An example rearranging the quadrants of a Fourier image can be found at
/// opencv_source/samples/python/dft.py
/// ## Parameters
/// * src: input array that could be real or complex.
/// * dst: output array whose size and type depends on the flags .
/// * flags: transformation flags, representing a combination of the #DftFlags
/// * nonzeroRows: when the parameter is not zero, the function assumes that only the first
/// nonzeroRows rows of the input array (#DFT_INVERSE is not set) or only the first nonzeroRows of the
/// output array (#DFT_INVERSE is set) contain non-zeros, thus, the function can handle the rest of the
/// rows more efficiently and save some time; this technique is very useful for calculating array
/// cross-correlation or convolution using DFT.
/// ## See also
/// dct , getOptimalDFTSize , mulSpectrums, filter2D , matchTemplate , flip , cartToPolar ,
/// magnitude , phase
///
/// ## C++ default parameters
/// * flags: 0
/// * nonzero_rows: 0
pub fn dft(src: &core::Mat, dst: &mut core::Mat, flags: i32, nonzero_rows: i32) -> Result<()> {
    unsafe { sys::cv_dft_Mat_Mat_int_int(src.as_raw_Mat(), dst.as_raw_Mat(), flags, nonzero_rows) }.into_result()
}

/// Get OpenCV type from DirectX type
/// ## Parameters
/// * iD3DFORMAT: - enum D3DTYPE for D3D9
/// ## Returns
/// OpenCV type or -1 if there is no equivalent
pub fn get_type_from_d3d_format(i_d3_dformat: i32) -> Result<i32> {
    unsafe { sys::cv_directx_getTypeFromD3DFORMAT_int(i_d3_dformat) }.into_result()
}

/// Get OpenCV type from DirectX type
/// ## Parameters
/// * iDXGI_FORMAT: - enum DXGI_FORMAT for D3D10/D3D11
/// ## Returns
/// OpenCV type or -1 if there is no equivalent
pub fn get_type_from_dxgi_format(i_dxgi_format: i32) -> Result<i32> {
    unsafe { sys::cv_directx_getTypeFromDXGI_FORMAT_int(i_dxgi_format) }.into_result()
}

/// Integer division with result round up.
/// 
/// Use this function instead of `ceil((float)a / b)` expressions.
/// 
/// ## See also
/// alignSize
pub fn div_up(a: i32, b: u32) -> Result<i32> {
    unsafe { sys::cv_divUp_int_unsigned_int(a, b) }.into_result()
}

/// Integer division with result round up.
/// 
/// Use this function instead of `ceil((float)a / b)` expressions.
/// 
/// ## See also
/// alignSize
/// 
/// ## Overloaded parameters
pub fn duv_up_u(a: size_t, b: u32) -> Result<size_t> {
    unsafe { sys::cv_divUp_size_t_unsigned_int(a, b) }.into_result()
}

/// Performs per-element division of two arrays or a scalar by an array.
/// 
/// The function cv::divide divides one array by another:
/// <div lang='latex'>\texttt{dst(I) = saturate(src1(I)*scale/src2(I))}</div>
/// or a scalar by an array when there is no src1 :
/// <div lang='latex'>\texttt{dst(I) = saturate(scale/src2(I))}</div>
/// 
/// When src2(I) is zero, dst(I) will also be zero. Different channels of
/// multi-channel arrays are processed independently.
/// 
/// 
/// Note: Saturation is not applied when the output array has the depth CV_32S. You may even get
/// result of an incorrect sign in the case of overflow.
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and type as src1.
/// * scale: scalar factor.
/// * dst: output array of the same size and type as src2.
/// * dtype: optional depth of the output array; if -1, dst will have depth src2.depth(), but in
/// case of an array-by-array division, you can only pass -1 when src1.depth()==src2.depth().
/// ## See also
/// multiply, add, subtract
///
/// ## C++ default parameters
/// * scale: 1
/// * dtype: -1
pub fn divide_mat(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat, scale: f64, dtype: i32) -> Result<()> {
    unsafe { sys::cv_divide_Mat_Mat_Mat_double_int(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat(), scale, dtype) }.into_result()
}

/// Performs per-element division of two arrays or a scalar by an array.
/// 
/// The function cv::divide divides one array by another:
/// <div lang='latex'>\texttt{dst(I) = saturate(src1(I)*scale/src2(I))}</div>
/// or a scalar by an array when there is no src1 :
/// <div lang='latex'>\texttt{dst(I) = saturate(scale/src2(I))}</div>
/// 
/// When src2(I) is zero, dst(I) will also be zero. Different channels of
/// multi-channel arrays are processed independently.
/// 
/// 
/// Note: Saturation is not applied when the output array has the depth CV_32S. You may even get
/// result of an incorrect sign in the case of overflow.
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and type as src1.
/// * scale: scalar factor.
/// * dst: output array of the same size and type as src2.
/// * dtype: optional depth of the output array; if -1, dst will have depth src2.depth(), but in
/// case of an array-by-array division, you can only pass -1 when src1.depth()==src2.depth().
/// ## See also
/// multiply, add, subtract
/// 
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * dtype: -1
pub fn divide(scale: f64, src2: &core::Mat, dst: &mut core::Mat, dtype: i32) -> Result<()> {
    unsafe { sys::cv_divide_double_Mat_Mat_int(scale, src2.as_raw_Mat(), dst.as_raw_Mat(), dtype) }.into_result()
}

/// Calculates eigenvalues and eigenvectors of a non-symmetric matrix (real eigenvalues only).
/// 
/// 
/// Note: Assumes real eigenvalues.
/// 
/// The function calculates eigenvalues and eigenvectors (optional) of the square matrix src:
/// ```ignore
/// src*eigenvectors.row(i).t() = eigenvalues.at<srcType>(i)*eigenvectors.row(i).t()
/// ```
/// 
/// 
/// ## Parameters
/// * src: input matrix (CV_32FC1 or CV_64FC1 type).
/// * eigenvalues: output vector of eigenvalues (type is the same type as src).
/// * eigenvectors: output matrix of eigenvectors (type is the same type as src). The eigenvectors are stored as subsequent matrix rows, in the same order as the corresponding eigenvalues.
/// ## See also
/// eigen
pub fn eigen_non_symmetric(src: &core::Mat, eigenvalues: &mut core::Mat, eigenvectors: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_eigenNonSymmetric_Mat_Mat_Mat(src.as_raw_Mat(), eigenvalues.as_raw_Mat(), eigenvectors.as_raw_Mat()) }.into_result()
}

/// Calculates eigenvalues and eigenvectors of a symmetric matrix.
/// 
/// The function cv::eigen calculates just eigenvalues, or eigenvalues and eigenvectors of the symmetric
/// matrix src:
/// ```ignore
/// src*eigenvectors.row(i).t() = eigenvalues.at<srcType>(i)*eigenvectors.row(i).t()
/// ```
/// 
/// 
/// 
/// Note: Use cv::eigenNonSymmetric for calculation of real eigenvalues and eigenvectors of non-symmetric matrix.
/// 
/// ## Parameters
/// * src: input matrix that must have CV_32FC1 or CV_64FC1 type, square size and be symmetrical
/// (src ^T^ == src).
/// * eigenvalues: output vector of eigenvalues of the same type as src; the eigenvalues are stored
/// in the descending order.
/// * eigenvectors: output matrix of eigenvectors; it has the same size and type as src; the
/// eigenvectors are stored as subsequent matrix rows, in the same order as the corresponding
/// eigenvalues.
/// ## See also
/// eigenNonSymmetric, completeSymm , PCA
///
/// ## C++ default parameters
/// * eigenvectors: noArray()
pub fn eigen(src: &core::Mat, eigenvalues: &mut core::Mat, eigenvectors: &mut core::Mat) -> Result<bool> {
    unsafe { sys::cv_eigen_Mat_Mat_Mat(src.as_raw_Mat(), eigenvalues.as_raw_Mat(), eigenvectors.as_raw_Mat()) }.into_result()
}

pub fn error(_code: i32, _err: &str, _func: &str, _file: &str, _line: i32) -> Result<()> {
    string_arg!(_err);
    string_arg!(_func);
    string_arg!(_file);
    unsafe { sys::cv_error_int_String_const_char_X_const_char_X_int(_code, _err.as_ptr(), _func.as_ptr(), _file.as_ptr(), _line) }.into_result()
}

/// Calculates the exponent of every array element.
/// 
/// The function cv::exp calculates the exponent of every element of the input
/// array:
/// <div lang='latex'>\texttt{dst} [I] = e^{ src(I) }</div>
/// 
/// The maximum relative error is about 7e-6 for single-precision input and
/// less than 1e-10 for double-precision input. Currently, the function
/// converts denormalized values to zeros on output. Special values (NaN,
/// Inf) are not handled.
/// ## Parameters
/// * src: input array.
/// * dst: output array of the same size and type as src.
/// ## See also
/// log , cartToPolar , polarToCart , phase , pow , sqrt , magnitude
pub fn exp(src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_exp_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Extracts a single channel from src (coi is 0-based index)
/// ## Parameters
/// * src: input array
/// * dst: output array
/// * coi: index of channel to extract
/// ## See also
/// mixChannels, split
pub fn extract_channel(src: &core::Mat, dst: &mut core::Mat, coi: i32) -> Result<()> {
    unsafe { sys::cv_extractChannel_Mat_Mat_int(src.as_raw_Mat(), dst.as_raw_Mat(), coi) }.into_result()
}

/// Calculates the angle of a 2D vector in degrees.
/// 
/// The function fastAtan2 calculates the full-range angle of an input 2D vector. The angle is measured
/// in degrees and varies from 0 to 360 degrees. The accuracy is about 0.3 degrees.
/// ## Parameters
/// * x: x-coordinate of the vector.
/// * y: y-coordinate of the vector.
pub fn fast_atan2(y: f32, x: f32) -> Result<f32> {
    unsafe { sys::cv_fastAtan2_float_float(y, x) }.into_result()
}

/// Returns the list of locations of non-zero pixels
/// 
/// Given a binary matrix (likely returned from an operation such
/// as threshold(), compare(), >, ==, etc, return all of
/// the non-zero indices as a cv::Mat or std::vector<cv::Point> (x,y)
/// For example:
/// ```ignore{.cpp}
/// cv::Mat binaryImage; // input, binary image
/// cv::Mat locations;   // output, locations of non-zero pixels
/// cv::findNonZero(binaryImage, locations);
/// 
/// // access pixel coordinates
/// Point pnt = locations.at<Point>(i);
/// ```
/// 
/// or
/// ```ignore{.cpp}
/// cv::Mat binaryImage; // input, binary image
/// vector<Point> locations;   // output, locations of non-zero pixels
/// cv::findNonZero(binaryImage, locations);
/// 
/// // access pixel coordinates
/// Point pnt = locations[i];
/// ```
/// 
/// ## Parameters
/// * src: single-channel array (type CV_8UC1)
/// * idx: the output array, type of cv::Mat or std::vector<Point>, corresponding to non-zero indices in the input
pub fn find_non_zero(src: &core::Mat, idx: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_findNonZero_Mat_Mat(src.as_raw_Mat(), idx.as_raw_Mat()) }.into_result()
}

/// Flips a 2D array around vertical, horizontal, or both axes.
/// 
/// The function cv::flip flips the array in one of three different ways (row
/// and column indices are 0-based):
/// <div lang='latex'>\texttt{dst} _{ij} =
/// \left\{
/// \begin{array}{l l}
/// \texttt{src} _{\texttt{src.rows}-i-1,j} & if\;  \texttt{flipCode} = 0 \\
/// \texttt{src} _{i, \texttt{src.cols} -j-1} & if\;  \texttt{flipCode} > 0 \\
/// \texttt{src} _{ \texttt{src.rows} -i-1, \texttt{src.cols} -j-1} & if\; \texttt{flipCode} < 0 \\
/// \end{array}
/// \right.</div>
/// The example scenarios of using the function are the following:
///   Vertical flipping of the image (flipCode == 0) to switch between
/// top-left and bottom-left image origin. This is a typical operation
/// in video processing on Microsoft Windows\* OS.
///   Horizontal flipping of the image with the subsequent horizontal
/// shift and absolute difference calculation to check for a
/// vertical-axis symmetry (flipCode \> 0).
///   Simultaneous horizontal and vertical flipping of the image with
/// the subsequent shift and absolute difference calculation to check
/// for a central symmetry (flipCode \< 0).
///   Reversing the order of point arrays (flipCode \> 0 or
/// flipCode == 0).
/// ## Parameters
/// * src: input array.
/// * dst: output array of the same size and type as src.
/// * flipCode: a flag to specify how to flip the array; 0 means
/// flipping around the x-axis and positive value (for example, 1) means
/// flipping around y-axis. Negative value (for example, -1) means flipping
/// around both axes.
/// ## See also
/// transpose , repeat , completeSymm
pub fn flip(src: &core::Mat, dst: &mut core::Mat, flip_code: i32) -> Result<()> {
    unsafe { sys::cv_flip_Mat_Mat_int(src.as_raw_Mat(), dst.as_raw_Mat(), flip_code) }.into_result()
}

/// Performs generalized matrix multiplication.
/// 
/// The function cv::gemm performs generalized matrix multiplication similar to the
/// gemm functions in BLAS level 3. For example,
/// `gemm(src1, src2, alpha, src3, beta, dst, GEMM_1_T + GEMM_3_T)`
/// corresponds to
/// <div lang='latex'>\texttt{dst} =  \texttt{alpha} \cdot \texttt{src1} ^T  \cdot \texttt{src2} +  \texttt{beta} \cdot \texttt{src3} ^T</div>
/// 
/// In case of complex (two-channel) data, performed a complex matrix
/// multiplication.
/// 
/// The function can be replaced with a matrix expression. For example, the
/// above call can be replaced with:
/// ```ignore{.cpp}
/// dst = alpha*src1.t()*src2 + beta*src3.t();
/// ```
/// 
/// ## Parameters
/// * src1: first multiplied input matrix that could be real(CV_32FC1,
/// CV_64FC1) or complex(CV_32FC2, CV_64FC2).
/// * src2: second multiplied input matrix of the same type as src1.
/// * alpha: weight of the matrix product.
/// * src3: third optional delta matrix added to the matrix product; it
/// should have the same type as src1 and src2.
/// * beta: weight of src3.
/// * dst: output matrix; it has the proper size and the same type as
/// input matrices.
/// * flags: operation flags (cv::GemmFlags)
/// ## See also
/// mulTransposed , transform
///
/// ## C++ default parameters
/// * flags: 0
pub fn gemm(src1: &core::Mat, src2: &core::Mat, alpha: f64, src3: &core::Mat, beta: f64, dst: &mut core::Mat, flags: i32) -> Result<()> {
    unsafe { sys::cv_gemm_Mat_Mat_double_Mat_double_Mat_int(src1.as_raw_Mat(), src2.as_raw_Mat(), alpha, src3.as_raw_Mat(), beta, dst.as_raw_Mat(), flags) }.into_result()
}

/// Returns full configuration time cmake output.
/// 
/// Returned value is raw cmake output including version control system revision, compiler version,
/// compiler flags, enabled modules and third party libraries, etc. Output format depends on target
/// architecture.
pub fn get_build_information() -> Result<String> {
    unsafe { sys::cv_getBuildInformation() }.into_result().map(crate::templ::receive_string)
}

/// Returns list of CPU features enabled during compilation.
/// 
/// Returned value is a string containing space separated list of CPU features with following markers:
/// 
/// - no markers - baseline features
/// - prefix `*` - features enabled in dispatcher
/// - suffix `?` - features enabled but not available in HW
/// 
/// Example: `SSE SSE2 SSE3 *SSE4.1 *SSE4.2 *FP16 *AVX *AVX2 *AVX512-SKX?`
pub fn get_cpu_features_line() -> Result<String> {
    unsafe { sys::cv_getCPUFeaturesLine() }.into_result().map(crate::templ::receive_string_mut)
}

/// Returns the number of CPU ticks.
/// 
/// The function returns the current number of CPU ticks on some architectures (such as x86, x64,
/// PowerPC). On other platforms the function is equivalent to getTickCount. It can also be used for
/// very accurate time measurements, as well as for RNG initialization. Note that in case of multi-CPU
/// systems a thread, from which getCPUTickCount is called, can be suspended and resumed at another CPU
/// with its own counter. So, theoretically (and practically) the subsequent calls to the function do
/// not necessary return the monotonously increasing values. Also, since a modern CPU varies the CPU
/// frequency depending on the load, the number of CPU clocks spent in some code cannot be directly
/// converted to time units. Therefore, getTickCount is generally a preferable solution for measuring
/// execution time.
pub fn get_cpu_tick_count() -> Result<i64> {
    unsafe { sys::cv_getCPUTickCount() }.into_result()
}

pub fn get_elem_size(_type: i32) -> Result<size_t> {
    unsafe { sys::cv_getElemSize_int(_type) }.into_result()
}

/// Returns feature name by ID
/// 
/// Returns empty string if feature is not defined
pub fn get_hardware_feature_name(feature: i32) -> Result<String> {
    unsafe { sys::cv_getHardwareFeatureName_int(feature) }.into_result().map(crate::templ::receive_string_mut)
}

/// Returns the number of threads used by OpenCV for parallel regions.
/// 
/// Always returns 1 if OpenCV is built without threading support.
/// 
/// The exact meaning of return value depends on the threading framework used by OpenCV library:
/// - `TBB` - The number of threads, that OpenCV will try to use for parallel regions. If there is
/// any tbb::thread_scheduler_init in user code conflicting with OpenCV, then function returns
/// default number of threads used by TBB library.
/// - `OpenMP` - An upper bound on the number of threads that could be used to form a new team.
/// - `Concurrency` - The number of threads, that OpenCV will try to use for parallel regions.
/// - `GCD` - Unsupported; returns the GCD thread pool limit (512) for compatibility.
/// - `C=` - The number of threads, that OpenCV will try to use for parallel regions, if before
/// called setNumThreads with threads \> 0, otherwise returns the number of logical CPUs,
/// available for the process.
/// ## See also
/// setNumThreads, getThreadNum
pub fn get_num_threads() -> Result<i32> {
    unsafe { sys::cv_getNumThreads() }.into_result()
}

/// Returns the number of logical CPUs available for the process.
pub fn get_number_of_cpus() -> Result<i32> {
    unsafe { sys::cv_getNumberOfCPUs() }.into_result()
}

/// Returns the optimal DFT size for a given vector size.
/// 
/// DFT performance is not a monotonic function of a vector size. Therefore, when you calculate
/// convolution of two arrays or perform the spectral analysis of an array, it usually makes sense to
/// pad the input data with zeros to get a bit larger array that can be transformed much faster than the
/// original one. Arrays whose size is a power-of-two (2, 4, 8, 16, 32, ...) are the fastest to process.
/// Though, the arrays whose size is a product of 2's, 3's, and 5's (for example, 300 = 5\*5\*3\*2\*2)
/// are also processed quite efficiently.
/// 
/// The function cv::getOptimalDFTSize returns the minimum number N that is greater than or equal to vecsize
/// so that the DFT of a vector of size N can be processed efficiently. In the current implementation N
/// = 2 ^p^ \* 3 ^q^ \* 5 ^r^ for some integer p, q, r.
/// 
/// The function returns a negative number if vecsize is too large (very close to INT_MAX ).
/// 
/// While the function cannot be used directly to estimate the optimal vector size for DCT transform
/// (since the current DCT implementation supports only even-size vectors), it can be easily processed
/// as getOptimalDFTSize((vecsize+1)/2)\*2.
/// ## Parameters
/// * vecsize: vector size.
/// ## See also
/// dft , dct , idft , idct , mulSpectrums
pub fn get_optimal_dft_size(vecsize: i32) -> Result<i32> {
    unsafe { sys::cv_getOptimalDFTSize_int(vecsize) }.into_result()
}

/// Returns the index of the currently executed thread within the current parallel region. Always
/// returns 0 if called outside of parallel region.
/// 
/// **Deprecated**: Current implementation doesn't corresponding to this documentation.
/// 
/// 
/// The exact meaning of the return value depends on the threading framework used by OpenCV library:
/// - `TBB` - Unsupported with current 4.1 TBB release. Maybe will be supported in future.
/// - `OpenMP` - The thread number, within the current team, of the calling thread.
/// - `Concurrency` - An ID for the virtual processor that the current context is executing on (0
/// for master thread and unique number for others, but not necessary 1,2,3,...).
/// - `GCD` - System calling thread's ID. Never returns 0 inside parallel region.
/// - `C=` - The index of the current parallel task.
/// ## See also
/// setNumThreads, getNumThreads
#[deprecated = "Current implementation doesn't corresponding to this documentation."]
pub fn get_thread_num() -> Result<i32> {
    unsafe { sys::cv_getThreadNum() }.into_result()
}

/// Returns the number of ticks.
/// 
/// The function returns the number of ticks after the certain event (for example, when the machine was
/// turned on). It can be used to initialize RNG or to measure a function execution time by reading the
/// tick count before and after the function call.
/// ## See also
/// getTickFrequency, TickMeter
pub fn get_tick_count() -> Result<i64> {
    unsafe { sys::cv_getTickCount() }.into_result()
}

/// Returns the number of ticks per second.
/// 
/// The function returns the number of ticks per second. That is, the following code computes the
/// execution time in seconds:
/// ```ignore
/// double t = (double)getTickCount();
/// // do something ...
/// t = ((double)getTickCount() - t)/getTickFrequency();
/// ```
/// 
/// ## See also
/// getTickCount, TickMeter
pub fn get_tick_frequency() -> Result<f64> {
    unsafe { sys::cv_getTickFrequency() }.into_result()
}

/// Returns major library version
pub fn get_version_major() -> Result<i32> {
    unsafe { sys::cv_getVersionMajor() }.into_result()
}

/// Returns minor library version
pub fn get_version_minor() -> Result<i32> {
    unsafe { sys::cv_getVersionMinor() }.into_result()
}

/// Returns revision field of the library version
pub fn get_version_revision() -> Result<i32> {
    unsafe { sys::cv_getVersionRevision() }.into_result()
}

/// Returns library version string
/// 
/// For example "3.4.1-dev".
/// 
/// ## See also
/// getMajorVersion, getMinorVersion, getRevisionVersion
pub fn get_version_string() -> Result<String> {
    unsafe { sys::cv_getVersionString() }.into_result().map(crate::templ::receive_string_mut)
}

///
/// ## C++ default parameters
/// * recursive: false
pub fn glob(pattern: &str, result: &mut types::VectorOfString, recursive: bool) -> Result<()> {
    string_arg!(mut pattern);
    unsafe { sys::cv_glob_String_VectorOfString_bool(pattern.as_ptr() as _, result.as_raw_VectorOfString(), recursive) }.into_result()
}

pub fn have_open_vx() -> Result<bool> {
    unsafe { sys::cv_haveOpenVX() }.into_result()
}

/// Applies horizontal concatenation to given matrices.
/// 
/// The function horizontally concatenates two or more cv::Mat matrices (with the same number of rows).
/// ```ignore{.cpp}
/// cv::Mat matArray[] = { cv::Mat(4, 1, CV_8UC1, cv::Scalar(1)),
/// cv::Mat(4, 1, CV_8UC1, cv::Scalar(2)),
/// cv::Mat(4, 1, CV_8UC1, cv::Scalar(3)),};
/// 
/// cv::Mat out;
/// cv::hconcat( matArray, 3, out );
/// //out:
/// //[1, 2, 3;
/// // 1, 2, 3;
/// // 1, 2, 3;
/// // 1, 2, 3]
/// ```
/// 
/// ## Parameters
/// * src: input array or vector of matrices. all of the matrices must have the same number of rows and the same depth.
/// * nsrc: number of matrices in src.
/// * dst: output array. It has the same number of rows and depth as the src, and the sum of cols of the src.
/// ## See also
/// cv::vconcat(const Mat*, size_t, OutputArray),  cv::vconcat(InputArrayOfArrays, OutputArray) and  cv::vconcat(InputArray, InputArray, OutputArray)
/// 
/// ## Overloaded parameters
/// 
/// ```ignore{.cpp}
/// cv::Mat_<float> A = (cv::Mat_<float>(3, 2) << 1, 4,
/// 2, 5,
/// 3, 6);
/// cv::Mat_<float> B = (cv::Mat_<float>(3, 2) << 7, 10,
/// 8, 11,
/// 9, 12);
/// 
/// cv::Mat C;
/// cv::hconcat(A, B, C);
/// //C:
/// //[1, 4, 7, 10;
/// // 2, 5, 8, 11;
/// // 3, 6, 9, 12]
/// ```
/// 
/// * src1: first input array to be considered for horizontal concatenation.
/// * src2: second input array to be considered for horizontal concatenation.
/// * dst: output array. It has the same number of rows and depth as the src1 and src2, and the sum of cols of the src1 and src2.
pub fn hconcat_2(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_hconcat_Mat_Mat_Mat(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Applies horizontal concatenation to given matrices.
/// 
/// The function horizontally concatenates two or more cv::Mat matrices (with the same number of rows).
/// ```ignore{.cpp}
/// cv::Mat matArray[] = { cv::Mat(4, 1, CV_8UC1, cv::Scalar(1)),
/// cv::Mat(4, 1, CV_8UC1, cv::Scalar(2)),
/// cv::Mat(4, 1, CV_8UC1, cv::Scalar(3)),};
/// 
/// cv::Mat out;
/// cv::hconcat( matArray, 3, out );
/// //out:
/// //[1, 2, 3;
/// // 1, 2, 3;
/// // 1, 2, 3;
/// // 1, 2, 3]
/// ```
/// 
/// ## Parameters
/// * src: input array or vector of matrices. all of the matrices must have the same number of rows and the same depth.
/// * nsrc: number of matrices in src.
/// * dst: output array. It has the same number of rows and depth as the src, and the sum of cols of the src.
/// ## See also
/// cv::vconcat(const Mat*, size_t, OutputArray),  cv::vconcat(InputArrayOfArrays, OutputArray) and  cv::vconcat(InputArray, InputArray, OutputArray)
/// 
/// ## Overloaded parameters
/// 
/// ```ignore{.cpp}
/// std::vector<cv::Mat> matrices = { cv::Mat(4, 1, CV_8UC1, cv::Scalar(1)),
/// cv::Mat(4, 1, CV_8UC1, cv::Scalar(2)),
/// cv::Mat(4, 1, CV_8UC1, cv::Scalar(3)),};
/// 
/// cv::Mat out;
/// cv::hconcat( matrices, out );
/// //out:
/// //[1, 2, 3;
/// // 1, 2, 3;
/// // 1, 2, 3;
/// // 1, 2, 3]
/// ```
/// 
/// * src: input array or vector of matrices. all of the matrices must have the same number of rows and the same depth.
/// * dst: output array. It has the same number of rows and depth as the src, and the sum of cols of the src.
/// same depth.
pub fn hconcat(src: &types::VectorOfMat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_hconcat_VectorOfMat_Mat(src.as_raw_VectorOfMat(), dst.as_raw_Mat()) }.into_result()
}

/// Calculates the inverse Discrete Cosine Transform of a 1D or 2D array.
/// 
/// idct(src, dst, flags) is equivalent to dct(src, dst, flags | DCT_INVERSE).
/// ## Parameters
/// * src: input floating-point single-channel array.
/// * dst: output array of the same size and type as src.
/// * flags: operation flags.
/// ## See also
/// dct, dft, idft, getOptimalDFTSize
///
/// ## C++ default parameters
/// * flags: 0
pub fn idct(src: &core::Mat, dst: &mut core::Mat, flags: i32) -> Result<()> {
    unsafe { sys::cv_idct_Mat_Mat_int(src.as_raw_Mat(), dst.as_raw_Mat(), flags) }.into_result()
}

/// Calculates the inverse Discrete Fourier Transform of a 1D or 2D array.
/// 
/// idft(src, dst, flags) is equivalent to dft(src, dst, flags | #DFT_INVERSE) .
/// 
/// Note: None of dft and idft scales the result by default. So, you should pass #DFT_SCALE to one of
/// dft or idft explicitly to make these transforms mutually inverse.
/// ## See also
/// dft, dct, idct, mulSpectrums, getOptimalDFTSize
/// ## Parameters
/// * src: input floating-point real or complex array.
/// * dst: output array whose size and type depend on the flags.
/// * flags: operation flags (see dft and #DftFlags).
/// * nonzeroRows: number of dst rows to process; the rest of the rows have undefined content (see
/// the convolution sample in dft description.
///
/// ## C++ default parameters
/// * flags: 0
/// * nonzero_rows: 0
pub fn idft(src: &core::Mat, dst: &mut core::Mat, flags: i32, nonzero_rows: i32) -> Result<()> {
    unsafe { sys::cv_idft_Mat_Mat_int_int(src.as_raw_Mat(), dst.as_raw_Mat(), flags, nonzero_rows) }.into_result()
}

/// Checks if array elements lie between the elements of two other arrays.
/// 
/// The function checks the range as follows:
/// *   For every element of a single-channel input array:
/// <div lang='latex'>\texttt{dst} (I)= \texttt{lowerb} (I)_0  \leq \texttt{src} (I)_0 \leq  \texttt{upperb} (I)_0</div>
/// *   For two-channel arrays:
/// <div lang='latex'>\texttt{dst} (I)= \texttt{lowerb} (I)_0  \leq \texttt{src} (I)_0 \leq  \texttt{upperb} (I)_0  \land \texttt{lowerb} (I)_1  \leq \texttt{src} (I)_1 \leq  \texttt{upperb} (I)_1</div>
/// *   and so forth.
/// 
/// That is, dst (I) is set to 255 (all 1 -bits) if src (I) is within the
/// specified 1D, 2D, 3D, ... box and 0 otherwise.
/// 
/// When the lower and/or upper boundary parameters are scalars, the indexes
/// (I) at lowerb and upperb in the above formulas should be omitted.
/// ## Parameters
/// * src: first input array.
/// * lowerb: inclusive lower boundary array or a scalar.
/// * upperb: inclusive upper boundary array or a scalar.
/// * dst: output array of the same size as src and CV_8U type.
pub fn in_range(src: &core::Mat, lowerb: &core::Mat, upperb: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_inRange_Mat_Mat_Mat_Mat(src.as_raw_Mat(), lowerb.as_raw_Mat(), upperb.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Inserts a single channel to dst (coi is 0-based index)
/// ## Parameters
/// * src: input array
/// * dst: output array
/// * coi: index of channel for insertion
/// ## See also
/// mixChannels, merge
pub fn insert_channel(src: &core::Mat, dst: &mut core::Mat, coi: i32) -> Result<()> {
    unsafe { sys::cv_insertChannel_Mat_Mat_int(src.as_raw_Mat(), dst.as_raw_Mat(), coi) }.into_result()
}

pub fn reset_trace() -> Result<()> {
    unsafe { sys::cv_instr_resetTrace() }.into_result()
}

pub fn set_flags(mode_flags: i32) -> Result<()> {
    unsafe { sys::cv_instr_setFlags_int(mode_flags) }.into_result()
}

pub fn set_use_instrumentation(flag: bool) -> Result<()> {
    unsafe { sys::cv_instr_setUseInstrumentation_bool(flag) }.into_result()
}

pub fn use_instrumentation() -> Result<bool> {
    unsafe { sys::cv_instr_useInstrumentation() }.into_result()
}

/// Finds the inverse or pseudo-inverse of a matrix.
/// 
/// The function cv::invert inverts the matrix src and stores the result in dst
/// . When the matrix src is singular or non-square, the function calculates
/// the pseudo-inverse matrix (the dst matrix) so that norm(src\*dst - I) is
/// minimal, where I is an identity matrix.
/// 
/// In case of the #DECOMP_LU method, the function returns non-zero value if
/// the inverse has been successfully calculated and 0 if src is singular.
/// 
/// In case of the #DECOMP_SVD method, the function returns the inverse
/// condition number of src (the ratio of the smallest singular value to the
/// largest singular value) and 0 if src is singular. The SVD method
/// calculates a pseudo-inverse matrix if src is singular.
/// 
/// Similarly to #DECOMP_LU, the method #DECOMP_CHOLESKY works only with
/// non-singular square matrices that should also be symmetrical and
/// positively defined. In this case, the function stores the inverted
/// matrix in dst and returns non-zero. Otherwise, it returns 0.
/// 
/// ## Parameters
/// * src: input floating-point M x N matrix.
/// * dst: output matrix of N x M size and the same type as src.
/// * flags: inversion method (cv::DecompTypes)
/// ## See also
/// solve, SVD
///
/// ## C++ default parameters
/// * flags: DECOMP_LU
pub fn invert(src: &core::Mat, dst: &mut core::Mat, flags: i32) -> Result<f64> {
    unsafe { sys::cv_invert_Mat_Mat_int(src.as_raw_Mat(), dst.as_raw_Mat(), flags) }.into_result()
}

pub fn get_ipp_error_location() -> Result<String> {
    unsafe { sys::cv_ipp_getIppErrorLocation() }.into_result().map(crate::templ::receive_string_mut)
}

pub fn get_ipp_features() -> Result<u64> {
    unsafe { sys::cv_ipp_getIppFeatures() }.into_result()
}

pub fn get_ipp_status() -> Result<i32> {
    unsafe { sys::cv_ipp_getIppStatus() }.into_result()
}

pub fn get_ipp_version() -> Result<String> {
    unsafe { sys::cv_ipp_getIppVersion() }.into_result().map(crate::templ::receive_string_mut)
}

pub fn set_use_ipp_ne(flag: bool) -> Result<()> {
    unsafe { sys::cv_ipp_setUseIPP_NE_bool(flag) }.into_result()
}

pub fn set_use_ipp__not_exact(flag: bool) -> Result<()> {
    unsafe { sys::cv_ipp_setUseIPP_NotExact_bool(flag) }.into_result()
}

pub fn set_use_ipp(flag: bool) -> Result<()> {
    unsafe { sys::cv_ipp_setUseIPP_bool(flag) }.into_result()
}

pub fn use_ipp() -> Result<bool> {
    unsafe { sys::cv_ipp_useIPP() }.into_result()
}

pub fn use_ipp_ne() -> Result<bool> {
    unsafe { sys::cv_ipp_useIPP_NE() }.into_result()
}

pub fn use_ipp__not_exact() -> Result<bool> {
    unsafe { sys::cv_ipp_useIPP_NotExact() }.into_result()
}

/// Finds centers of clusters and groups input samples around the clusters.
/// 
/// The function kmeans implements a k-means algorithm that finds the centers of cluster_count clusters
/// and groups the input samples around the clusters. As an output, <span lang='latex'>\texttt{bestLabels}_i</span> contains a
/// 0-based cluster index for the sample stored in the <span lang='latex'>i^{th}</span> row of the samples matrix.
/// 
/// 
/// Note:
/// *   (Python) An example on K-means clustering can be found at
/// opencv_source_code/samples/python/kmeans.py
/// ## Parameters
/// * data: Data for clustering. An array of N-Dimensional points with float coordinates is needed.
/// Examples of this array can be:
/// *   Mat points(count, 2, CV_32F);
/// *   Mat points(count, 1, CV_32FC2);
/// *   Mat points(1, count, CV_32FC2);
/// *   std::vector\<cv::Point2f\> points(sampleCount);
/// * K: Number of clusters to split the set by.
/// * bestLabels: Input/output integer array that stores the cluster indices for every sample.
/// * criteria: The algorithm termination criteria, that is, the maximum number of iterations and/or
/// the desired accuracy. The accuracy is specified as criteria.epsilon. As soon as each of the cluster
/// centers moves by less than criteria.epsilon on some iteration, the algorithm stops.
/// * attempts: Flag to specify the number of times the algorithm is executed using different
/// initial labellings. The algorithm returns the labels that yield the best compactness (see the last
/// function parameter).
/// * flags: Flag that can take values of cv::KmeansFlags
/// * centers: Output matrix of the cluster centers, one row per each cluster center.
/// ## Returns
/// The function returns the compactness measure that is computed as
/// <div lang='latex'>\sum _i  \| \texttt{samples} _i -  \texttt{centers} _{ \texttt{labels} _i} \| ^2</div>
/// after every attempt. The best (minimum) value is chosen and the corresponding labels and the
/// compactness value are returned by the function. Basically, you can use only the core of the
/// function, set the number of attempts to 1, initialize labels each time using a custom algorithm,
/// pass them with the ( flags = #KMEANS_USE_INITIAL_LABELS ) flag, and then choose the best
/// (most-compact) clustering.
///
/// ## C++ default parameters
/// * centers: noArray()
pub fn kmeans(data: &core::Mat, k: i32, best_labels: &mut core::Mat, criteria: &core::TermCriteria, attempts: i32, flags: i32, centers: &mut core::Mat) -> Result<f64> {
    unsafe { sys::cv_kmeans_Mat_int_Mat_TermCriteria_int_int_Mat(data.as_raw_Mat(), k, best_labels.as_raw_Mat(), criteria.as_raw_TermCriteria(), attempts, flags, centers.as_raw_Mat()) }.into_result()
}

/// Calculates the natural logarithm of every array element.
/// 
/// The function cv::log calculates the natural logarithm of every element of the input array:
/// <div lang='latex'>\texttt{dst} (I) =  \log (\texttt{src}(I)) </div>
/// 
/// Output on zero, negative and special (NaN, Inf) values is undefined.
/// 
/// ## Parameters
/// * src: input array.
/// * dst: output array of the same size and type as src .
/// ## See also
/// exp, cartToPolar, polarToCart, phase, pow, sqrt, magnitude
pub fn log(src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_log_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Calculates the magnitude of 2D vectors.
/// 
/// The function cv::magnitude calculates the magnitude of 2D vectors formed
/// from the corresponding elements of x and y arrays:
/// <div lang='latex'>\texttt{dst} (I) =  \sqrt{\texttt{x}(I)^2 + \texttt{y}(I)^2}</div>
/// ## Parameters
/// * x: floating-point array of x-coordinates of the vectors.
/// * y: floating-point array of y-coordinates of the vectors; it must
/// have the same size as x.
/// * magnitude: output array of the same size and type as x.
/// ## See also
/// cartToPolar, polarToCart, phase, sqrt
pub fn magnitude(x: &core::Mat, y: &core::Mat, magnitude: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_magnitude_Mat_Mat_Mat(x.as_raw_Mat(), y.as_raw_Mat(), magnitude.as_raw_Mat()) }.into_result()
}

/// Calculates per-element maximum of two arrays or an array and a scalar.
/// 
/// The function cv::max calculates the per-element maximum of two arrays:
/// <div lang='latex'>\texttt{dst} (I)= \max ( \texttt{src1} (I), \texttt{src2} (I))</div>
/// or array and a scalar:
/// <div lang='latex'>\texttt{dst} (I)= \max ( \texttt{src1} (I), \texttt{value} )</div>
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and type as src1 .
/// * dst: output array of the same size and type as src1.
/// ## See also
/// min, compare, inRange, minMaxLoc, @ref MatrixExpressions
pub fn max(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_max_Mat_Mat_Mat(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Calculates a mean and standard deviation of array elements.
/// 
/// The function cv::meanStdDev calculates the mean and the standard deviation M
/// of array elements independently for each channel and returns it via the
/// output parameters:
/// <div lang='latex'>\begin{array}{l} N =  \sum _{I, \texttt{mask} (I)  \ne 0} 1 \\ \texttt{mean} _c =  \frac{\sum_{ I: \; \texttt{mask}(I) \ne 0} \texttt{src} (I)_c}{N} \\ \texttt{stddev} _c =  \sqrt{\frac{\sum_{ I: \; \texttt{mask}(I) \ne 0} \left ( \texttt{src} (I)_c -  \texttt{mean} _c \right )^2}{N}} \end{array}</div>
/// When all the mask elements are 0's, the function returns
/// mean=stddev=Scalar::all(0).
/// 
/// Note: The calculated standard deviation is only the diagonal of the
/// complete normalized covariance matrix. If the full matrix is needed, you
/// can reshape the multi-channel array M x N to the single-channel array
/// M\*N x mtx.channels() (only possible when the matrix is continuous) and
/// then pass the matrix to calcCovarMatrix .
/// ## Parameters
/// * src: input array that should have from 1 to 4 channels so that the results can be stored in
/// Scalar_ 's.
/// * mean: output parameter: calculated mean value.
/// * stddev: output parameter: calculated standard deviation.
/// * mask: optional operation mask.
/// ## See also
/// countNonZero, mean, norm, minMaxLoc, calcCovarMatrix
///
/// ## C++ default parameters
/// * mask: noArray()
pub fn mean_std_dev(src: &core::Mat, mean: &mut core::Mat, stddev: &mut core::Mat, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_meanStdDev_Mat_Mat_Mat_Mat(src.as_raw_Mat(), mean.as_raw_Mat(), stddev.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
}

/// Calculates an average (mean) of array elements.
/// 
/// The function cv::mean calculates the mean value M of array elements,
/// independently for each channel, and return it:
/// <div lang='latex'>\begin{array}{l} N =  \sum _{I: \; \texttt{mask} (I) \ne 0} 1 \\ M_c =  \left ( \sum _{I: \; \texttt{mask} (I) \ne 0}{ \texttt{mtx} (I)_c} \right )/N \end{array}</div>
/// When all the mask elements are 0's, the function returns Scalar::all(0)
/// ## Parameters
/// * src: input array that should have from 1 to 4 channels so that the result can be stored in
/// Scalar_ .
/// * mask: optional operation mask.
/// ## See also
/// countNonZero, meanStdDev, norm, minMaxLoc
///
/// ## C++ default parameters
/// * mask: noArray()
pub fn mean(src: &core::Mat, mask: &core::Mat) -> Result<core::Scalar> {
    unsafe { sys::cv_mean_Mat_Mat(src.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
}

/// Creates one multi-channel array out of several single-channel ones.
/// 
/// The function cv::merge merges several arrays to make a single multi-channel array. That is, each
/// element of the output array will be a concatenation of the elements of the input arrays, where
/// elements of i-th input array are treated as mv[i].channels()-element vectors.
/// 
/// The function cv::split does the reverse operation. If you need to shuffle channels in some other
/// advanced way, use cv::mixChannels.
/// 
/// The following example shows how to merge 3 single channel matrices into a single 3-channel matrix.
/// @snippet snippets/core_merge.cpp example
/// 
/// ## Parameters
/// * mv: input array of matrices to be merged; all the matrices in mv must have the same
/// size and the same depth.
/// * count: number of input matrices when mv is a plain C array; it must be greater than zero.
/// * dst: output array of the same size and the same depth as mv[0]; The number of channels will
/// be equal to the parameter count.
/// ## See also
/// mixChannels, split, Mat::reshape
/// 
/// ## Overloaded parameters
/// 
/// * mv: input vector of matrices to be merged; all the matrices in mv must have the same
/// size and the same depth.
/// * dst: output array of the same size and the same depth as mv[0]; The number of channels will
/// be the total number of channels in the matrix array.
pub fn merge(mv: &types::VectorOfMat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_merge_VectorOfMat_Mat(mv.as_raw_VectorOfMat(), dst.as_raw_Mat()) }.into_result()
}

/// Finds the global minimum and maximum in an array
/// 
/// The function cv::minMaxIdx finds the minimum and maximum element values and their positions. The
/// extremums are searched across the whole array or, if mask is not an empty array, in the specified
/// array region. The function does not work with multi-channel arrays. If you need to find minimum or
/// maximum elements across all the channels, use Mat::reshape first to reinterpret the array as
/// single-channel. Or you may extract the particular channel using either extractImageCOI , or
/// mixChannels , or split . In case of a sparse matrix, the minimum is found among non-zero elements
/// only.
/// 
/// Note: When minIdx is not NULL, it must have at least 2 elements (as well as maxIdx), even if src is
/// a single-row or single-column matrix. In OpenCV (following MATLAB) each array has at least 2
/// dimensions, i.e. single-column matrix is Mx1 matrix (and therefore minIdx/maxIdx will be
/// (i1,0)/(i2,0)) and single-row matrix is 1xN matrix (and therefore minIdx/maxIdx will be
/// (0,j1)/(0,j2)).
/// ## Parameters
/// * src: input single-channel array.
/// * minVal: pointer to the returned minimum value; NULL is used if not required.
/// * maxVal: pointer to the returned maximum value; NULL is used if not required.
/// * minIdx: pointer to the returned minimum location (in nD case); NULL is used if not required;
/// Otherwise, it must point to an array of src.dims elements, the coordinates of the minimum element
/// in each dimension are stored there sequentially.
/// * maxIdx: pointer to the returned maximum location (in nD case). NULL is used if not required.
/// * mask: specified array region
///
/// ## C++ default parameters
/// * max_val: 0
/// * min_idx: 0
/// * max_idx: 0
/// * mask: noArray()
pub fn min_max_idx(src: &core::Mat, min_val: &mut f64, max_val: &mut f64, min_idx: &mut i32, max_idx: &mut i32, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_minMaxIdx_Mat_double_X_double_X_int_X_int_X_Mat(src.as_raw_Mat(), min_val, max_val, min_idx, max_idx, mask.as_raw_Mat()) }.into_result()
}

/// Finds the global minimum and maximum in an array.
/// 
/// The function cv::minMaxLoc finds the minimum and maximum element values and their positions. The
/// extremums are searched across the whole array or, if mask is not an empty array, in the specified
/// array region.
/// 
/// The function do not work with multi-channel arrays. If you need to find minimum or maximum
/// elements across all the channels, use Mat::reshape first to reinterpret the array as
/// single-channel. Or you may extract the particular channel using either extractImageCOI , or
/// mixChannels , or split .
/// ## Parameters
/// * src: input single-channel array.
/// * minVal: pointer to the returned minimum value; NULL is used if not required.
/// * maxVal: pointer to the returned maximum value; NULL is used if not required.
/// * minLoc: pointer to the returned minimum location (in 2D case); NULL is used if not required.
/// * maxLoc: pointer to the returned maximum location (in 2D case); NULL is used if not required.
/// * mask: optional mask used to select a sub-array.
/// ## See also
/// max, min, compare, inRange, extractImageCOI, mixChannels, split, Mat::reshape
///
/// ## C++ default parameters
/// * max_val: 0
/// * min_loc: 0
/// * max_loc: 0
/// * mask: noArray()
pub fn min_max_loc(src: &core::Mat, min_val: &mut f64, max_val: &mut f64, min_loc: &mut core::Point, max_loc: &mut core::Point, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_minMaxLoc_Mat_double_X_double_X_Point_X_Point_X_Mat(src.as_raw_Mat(), min_val, max_val, min_loc, max_loc, mask.as_raw_Mat()) }.into_result()
}

/// Calculates per-element minimum of two arrays or an array and a scalar.
/// 
/// The function cv::min calculates the per-element minimum of two arrays:
/// <div lang='latex'>\texttt{dst} (I)= \min ( \texttt{src1} (I), \texttt{src2} (I))</div>
/// or array and a scalar:
/// <div lang='latex'>\texttt{dst} (I)= \min ( \texttt{src1} (I), \texttt{value} )</div>
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and type as src1.
/// * dst: output array of the same size and type as src1.
/// ## See also
/// max, compare, inRange, minMaxLoc
pub fn min(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_min_Mat_Mat_Mat(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Copies specified channels from input arrays to the specified channels of
/// output arrays.
/// 
/// The function cv::mixChannels provides an advanced mechanism for shuffling image channels.
/// 
/// cv::split,cv::merge,cv::extractChannel,cv::insertChannel and some forms of cv::cvtColor are partial cases of cv::mixChannels.
/// 
/// In the example below, the code splits a 4-channel BGRA image into a 3-channel BGR (with B and R
/// channels swapped) and a separate alpha-channel image:
/// ```ignore{.cpp}
/// Mat bgra( 100, 100, CV_8UC4, Scalar(255,0,0,255) );
/// Mat bgr( bgra.rows, bgra.cols, CV_8UC3 );
/// Mat alpha( bgra.rows, bgra.cols, CV_8UC1 );
/// 
/// // forming an array of matrices is a quite efficient operation,
/// // because the matrix data is not copied, only the headers
/// Mat out[] = { bgr, alpha };
/// // bgra[0] -> bgr[2], bgra[1] -> bgr[1],
/// // bgra[2] -> bgr[0], bgra[3] -> alpha[0]
/// int from_to[] = { 0,2, 1,1, 2,0, 3,3 };
/// mixChannels( &bgra, 1, out, 2, from_to, 4 );
/// ```
/// 
/// 
/// Note: Unlike many other new-style C++ functions in OpenCV (see the introduction section and
/// Mat::create ), cv::mixChannels requires the output arrays to be pre-allocated before calling the
/// function.
/// ## Parameters
/// * src: input array or vector of matrices; all of the matrices must have the same size and the
/// same depth.
/// * nsrcs: number of matrices in `src`.
/// * dst: output array or vector of matrices; all the matrices **must be allocated**; their size and
/// depth must be the same as in `src[0]`.
/// * ndsts: number of matrices in `dst`.
/// * fromTo: array of index pairs specifying which channels are copied and where; fromTo[k\*2] is
/// a 0-based index of the input channel in src, fromTo[k\*2+1] is an index of the output channel in
/// dst; the continuous channel numbering is used: the first input image channels are indexed from 0 to
/// src[0].channels()-1, the second input image channels are indexed from src[0].channels() to
/// src[0].channels() + src[1].channels()-1, and so on, the same scheme is used for the output image
/// channels; as a special case, when fromTo[k\*2] is negative, the corresponding output channel is
/// filled with zero .
/// * npairs: number of index pairs in `fromTo`.
/// ## See also
/// split, merge, extractChannel, insertChannel, cvtColor
/// 
/// ## Overloaded parameters
/// 
/// * src: input array or vector of matrices; all of the matrices must have the same size and the
/// same depth.
/// * dst: output array or vector of matrices; all the matrices **must be allocated**; their size and
/// depth must be the same as in src[0].
/// * fromTo: array of index pairs specifying which channels are copied and where; fromTo[k\*2] is
/// a 0-based index of the input channel in src, fromTo[k\*2+1] is an index of the output channel in
/// dst; the continuous channel numbering is used: the first input image channels are indexed from 0 to
/// src[0].channels()-1, the second input image channels are indexed from src[0].channels() to
/// src[0].channels() + src[1].channels()-1, and so on, the same scheme is used for the output image
/// channels; as a special case, when fromTo[k\*2] is negative, the corresponding output channel is
/// filled with zero .
pub fn mix_channels(src: &types::VectorOfMat, dst: &mut types::VectorOfMat, from_to: &types::VectorOfint) -> Result<()> {
    unsafe { sys::cv_mixChannels_VectorOfMat_VectorOfMat_VectorOfint(src.as_raw_VectorOfMat(), dst.as_raw_VectorOfMat(), from_to.as_raw_VectorOfint()) }.into_result()
}

/// Performs the per-element multiplication of two Fourier spectrums.
/// 
/// The function cv::mulSpectrums performs the per-element multiplication of the two CCS-packed or complex
/// matrices that are results of a real or complex Fourier transform.
/// 
/// The function, together with dft and idft , may be used to calculate convolution (pass conjB=false )
/// or correlation (pass conjB=true ) of two arrays rapidly. When the arrays are complex, they are
/// simply multiplied (per element) with an optional conjugation of the second-array elements. When the
/// arrays are real, they are assumed to be CCS-packed (see dft for details).
/// ## Parameters
/// * a: first input array.
/// * b: second input array of the same size and type as src1 .
/// * c: output array of the same size and type as src1 .
/// * flags: operation flags; currently, the only supported flag is cv::DFT_ROWS, which indicates that
/// each row of src1 and src2 is an independent 1D Fourier spectrum. If you do not want to use this flag, then simply add a `0` as value.
/// * conjB: optional flag that conjugates the second input array before the multiplication (true)
/// or not (false).
///
/// ## C++ default parameters
/// * conj_b: false
pub fn mul_spectrums(a: &core::Mat, b: &core::Mat, c: &mut core::Mat, flags: i32, conj_b: bool) -> Result<()> {
    unsafe { sys::cv_mulSpectrums_Mat_Mat_Mat_int_bool(a.as_raw_Mat(), b.as_raw_Mat(), c.as_raw_Mat(), flags, conj_b) }.into_result()
}

/// Calculates the product of a matrix and its transposition.
/// 
/// The function cv::mulTransposed calculates the product of src and its
/// transposition:
/// <div lang='latex'>\texttt{dst} = \texttt{scale} ( \texttt{src} - \texttt{delta} )^T ( \texttt{src} - \texttt{delta} )</div>
/// if aTa=true , and
/// <div lang='latex'>\texttt{dst} = \texttt{scale} ( \texttt{src} - \texttt{delta} ) ( \texttt{src} - \texttt{delta} )^T</div>
/// otherwise. The function is used to calculate the covariance matrix. With
/// zero delta, it can be used as a faster substitute for general matrix
/// product A\*B when B=A'
/// ## Parameters
/// * src: input single-channel matrix. Note that unlike gemm, the
/// function can multiply not only floating-point matrices.
/// * dst: output square matrix.
/// * aTa: Flag specifying the multiplication ordering. See the
/// description below.
/// * delta: Optional delta matrix subtracted from src before the
/// multiplication. When the matrix is empty ( delta=noArray() ), it is
/// assumed to be zero, that is, nothing is subtracted. If it has the same
/// size as src , it is simply subtracted. Otherwise, it is "repeated" (see
/// repeat ) to cover the full src and then subtracted. Type of the delta
/// matrix, when it is not empty, must be the same as the type of created
/// output matrix. See the dtype parameter description below.
/// * scale: Optional scale factor for the matrix product.
/// * dtype: Optional type of the output matrix. When it is negative,
/// the output matrix will have the same type as src . Otherwise, it will be
/// type=CV_MAT_DEPTH(dtype) that should be either CV_32F or CV_64F .
/// ## See also
/// calcCovarMatrix, gemm, repeat, reduce
///
/// ## C++ default parameters
/// * delta: noArray()
/// * scale: 1
/// * dtype: -1
pub fn mul_transposed(src: &core::Mat, dst: &mut core::Mat, a_ta: bool, delta: &core::Mat, scale: f64, dtype: i32) -> Result<()> {
    unsafe { sys::cv_mulTransposed_Mat_Mat_bool_Mat_double_int(src.as_raw_Mat(), dst.as_raw_Mat(), a_ta, delta.as_raw_Mat(), scale, dtype) }.into_result()
}

/// Calculates the per-element scaled product of two arrays.
/// 
/// The function multiply calculates the per-element product of two arrays:
/// 
/// <div lang='latex'>\texttt{dst} (I)= \texttt{saturate} ( \texttt{scale} \cdot \texttt{src1} (I)  \cdot \texttt{src2} (I))</div>
/// 
/// There is also a @ref MatrixExpressions -friendly variant of the first function. See Mat::mul .
/// 
/// For a not-per-element matrix product, see gemm .
/// 
/// 
/// Note: Saturation is not applied when the output array has the depth
/// CV_32S. You may even get result of an incorrect sign in the case of
/// overflow.
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and the same type as src1.
/// * dst: output array of the same size and type as src1.
/// * scale: optional scale factor.
/// * dtype: optional depth of the output array
/// ## See also
/// add, subtract, divide, scaleAdd, addWeighted, accumulate, accumulateProduct, accumulateSquare,
/// Mat::convertTo
///
/// ## C++ default parameters
/// * scale: 1
/// * dtype: -1
pub fn multiply(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat, scale: f64, dtype: i32) -> Result<()> {
    unsafe { sys::cv_multiply_Mat_Mat_Mat_double_int(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat(), scale, dtype) }.into_result()
}

pub fn norm_l1(a: &f32, b: &f32, n: i32) -> Result<f32> {
    unsafe { sys::cv_normL1_const_float_X_const_float_X_int(a, b, n) }.into_result()
}

pub fn norm_l2(a: &u8, b: &u8, n: i32) -> Result<i32> {
    unsafe { sys::cv_normL1_const_uchar_X_const_uchar_X_int(a, b, n) }.into_result()
}

pub fn norm_l2_sqr(a: &f32, b: &f32, n: i32) -> Result<f32> {
    unsafe { sys::cv_normL2Sqr_const_float_X_const_float_X_int(a, b, n) }.into_result()
}

/// Calculates an absolute difference norm or a relative difference norm.
/// 
/// This version of cv::norm calculates the absolute difference norm
/// or the relative difference norm of arrays src1 and src2.
/// The type of norm to calculate is specified using #NormTypes.
/// 
/// ## Parameters
/// * src1: first input array.
/// * src2: second input array of the same size and the same type as src1.
/// * normType: type of the norm (see #NormTypes).
/// * mask: optional operation mask; it must have the same size as src1 and CV_8UC1 type.
///
/// ## C++ default parameters
/// * norm_type: NORM_L2
/// * mask: noArray()
pub fn norm_with_type(src1: &core::Mat, src2: &core::Mat, norm_type: i32, mask: &core::Mat) -> Result<f64> {
    unsafe { sys::cv_norm_Mat_Mat_int_Mat(src1.as_raw_Mat(), src2.as_raw_Mat(), norm_type, mask.as_raw_Mat()) }.into_result()
}

/// Calculates the  absolute norm of an array.
/// 
/// This version of #norm calculates the absolute norm of src1. The type of norm to calculate is specified using #NormTypes.
/// 
/// As example for one array consider the function <span lang='latex'>r(x)= \begin{pmatrix} x \\ 1-x \end{pmatrix}, x \in [-1;1]</span>.
/// The <span lang='latex'> L_{1}, L_{2} </span> and <span lang='latex'> L_{\infty} </span> norm for the sample value <span lang='latex'>r(-1) = \begin{pmatrix} -1 \\ 2 \end{pmatrix}</span>
/// is calculated as follows
/// \f{align*}
/// \| r(-1) \|_{L_1} &= |-1| + |2| = 3 \\
/// \| r(-1) \|_{L_2} &= \sqrt{(-1)^{2} + (2)^{2}} = \sqrt{5} \\
/// \| r(-1) \|_{L_\infty} &= \max(|-1|,|2|) = 2
/// \f}
/// and for <span lang='latex'>r(0.5) = \begin{pmatrix} 0.5 \\ 0.5 \end{pmatrix}</span> the calculation is
/// \f{align*}
/// \| r(0.5) \|_{L_1} &= |0.5| + |0.5| = 1 \\
/// \| r(0.5) \|_{L_2} &= \sqrt{(0.5)^{2} + (0.5)^{2}} = \sqrt{0.5} \\
/// \| r(0.5) \|_{L_\infty} &= \max(|0.5|,|0.5|) = 0.5.
/// \f}
/// The following graphic shows all values for the three norm functions <span lang='latex'>\| r(x) \|_{L_1}, \| r(x) \|_{L_2}</span> and <span lang='latex'>\| r(x) \|_{L_\infty}</span>.
/// It is notable that the <span lang='latex'> L_{1} </span> norm forms the upper and the <span lang='latex'> L_{\infty} </span> norm forms the lower border for the example function <span lang='latex'> r(x) </span>.
/// ![Graphs for the different norm functions from the above example](https://docs.opencv.org/3.4.6/NormTypes_OneArray_1-2-INF.png)
/// 
/// When the mask parameter is specified and it is not empty, the norm is
/// 
/// If normType is not specified, #NORM_L2 is used.
/// calculated only over the region specified by the mask.
/// 
/// Multi-channel input arrays are treated as single-channel arrays, that is,
/// the results for all channels are combined.
/// 
/// Hamming norms can only be calculated with CV_8U depth arrays.
/// 
/// ## Parameters
/// * src1: first input array.
/// * normType: type of the norm (see #NormTypes).
/// * mask: optional operation mask; it must have the same size as src1 and CV_8UC1 type.
///
/// ## C++ default parameters
/// * norm_type: NORM_L2
/// * mask: noArray()
pub fn norm(src1: &core::Mat, norm_type: i32, mask: &core::Mat) -> Result<f64> {
    unsafe { sys::cv_norm_Mat_int_Mat(src1.as_raw_Mat(), norm_type, mask.as_raw_Mat()) }.into_result()
}

/// Normalizes the norm or value range of an array.
/// 
/// The function cv::normalize normalizes scale and shift the input array elements so that
/// <div lang='latex'>\| \texttt{dst} \| _{L_p}= \texttt{alpha}</div>
/// (where p=Inf, 1 or 2) when normType=NORM_INF, NORM_L1, or NORM_L2, respectively; or so that
/// <div lang='latex'>\min _I  \texttt{dst} (I)= \texttt{alpha} , \, \, \max _I  \texttt{dst} (I)= \texttt{beta}</div>
/// 
/// when normType=NORM_MINMAX (for dense arrays only). The optional mask specifies a sub-array to be
/// normalized. This means that the norm or min-n-max are calculated over the sub-array, and then this
/// sub-array is modified to be normalized. If you want to only use the mask to calculate the norm or
/// min-max but modify the whole array, you can use norm and Mat::convertTo.
/// 
/// In case of sparse matrices, only the non-zero values are analyzed and transformed. Because of this,
/// the range transformation for sparse matrices is not allowed since it can shift the zero level.
/// 
/// Possible usage with some positive example data:
/// ```ignore{.cpp}
/// vector<double> positiveData = { 2.0, 8.0, 10.0 };
/// vector<double> normalizedData_l1, normalizedData_l2, normalizedData_inf, normalizedData_minmax;
/// 
/// // Norm to probability (total count)
/// // sum(numbers) = 20.0
/// // 2.0      0.1     (2.0/20.0)
/// // 8.0      0.4     (8.0/20.0)
/// // 10.0     0.5     (10.0/20.0)
/// normalize(positiveData, normalizedData_l1, 1.0, 0.0, NORM_L1);
/// 
/// // Norm to unit vector: ||positiveData|| = 1.0
/// // 2.0      0.15
/// // 8.0      0.62
/// // 10.0     0.77
/// normalize(positiveData, normalizedData_l2, 1.0, 0.0, NORM_L2);
/// 
/// // Norm to max element
/// // 2.0      0.2     (2.0/10.0)
/// // 8.0      0.8     (8.0/10.0)
/// // 10.0     1.0     (10.0/10.0)
/// normalize(positiveData, normalizedData_inf, 1.0, 0.0, NORM_INF);
/// 
/// // Norm to range [0.0;1.0]
/// // 2.0      0.0     (shift to left border)
/// // 8.0      0.75    (6.0/8.0)
/// // 10.0     1.0     (shift to right border)
/// normalize(positiveData, normalizedData_minmax, 1.0, 0.0, NORM_MINMAX);
/// ```
/// 
/// 
/// ## Parameters
/// * src: input array.
/// * dst: output array of the same size as src .
/// * alpha: norm value to normalize to or the lower range boundary in case of the range
/// normalization.
/// * beta: upper range boundary in case of the range normalization; it is not used for the norm
/// normalization.
/// * norm_type: normalization type (see cv::NormTypes).
/// * dtype: when negative, the output array has the same type as src; otherwise, it has the same
/// number of channels as src and the depth =CV_MAT_DEPTH(dtype).
/// * mask: optional operation mask.
/// ## See also
/// norm, Mat::convertTo, SparseMat::convertTo
///
/// ## C++ default parameters
/// * alpha: 1
/// * beta: 0
/// * norm_type: NORM_L2
/// * dtype: -1
/// * mask: noArray()
pub fn normalize(src: &core::Mat, dst: &mut core::Mat, alpha: f64, beta: f64, norm_type: i32, dtype: i32, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_normalize_Mat_Mat_double_double_int_int_Mat(src.as_raw_Mat(), dst.as_raw_Mat(), alpha, beta, norm_type, dtype, mask.as_raw_Mat()) }.into_result()
}

/// Parallel data processor
///
/// ## C++ default parameters
/// * nstripes: -1.
pub fn parallel_for_(range: &core::Range, body: &core::ParallelLoopBody, nstripes: f64) -> Result<()> {
    unsafe { sys::cv_parallel_for__Range_ParallelLoopBody_double(range.as_raw_Range(), body.as_raw_ParallelLoopBody(), nstripes) }.into_result()
}

/// converts NaN's to the given number
///
/// ## C++ default parameters
/// * val: 0
pub fn patch_na_ns(a: &mut core::Mat, val: f64) -> Result<()> {
    unsafe { sys::cv_patchNaNs_Mat_double(a.as_raw_Mat(), val) }.into_result()
}

/// Performs the perspective matrix transformation of vectors.
/// 
/// The function cv::perspectiveTransform transforms every element of src by
/// treating it as a 2D or 3D vector, in the following way:
/// <div lang='latex'>(x, y, z)  \rightarrow (x'/w, y'/w, z'/w)</div>
/// where
/// <div lang='latex'>(x', y', z', w') =  \texttt{mat} \cdot \begin{bmatrix} x & y & z & 1  \end{bmatrix}</div>
/// and
/// <div lang='latex'>w =  \fork{w'}{if \(w' \ne 0\)}{\infty}{otherwise}</div>
/// 
/// Here a 3D vector transformation is shown. In case of a 2D vector
/// transformation, the z component is omitted.
/// 
/// 
/// Note: The function transforms a sparse set of 2D or 3D vectors. If you
/// want to transform an image using perspective transformation, use
/// warpPerspective . If you have an inverse problem, that is, you want to
/// compute the most probable perspective transformation out of several
/// pairs of corresponding points, you can use getPerspectiveTransform or
/// findHomography .
/// ## Parameters
/// * src: input two-channel or three-channel floating-point array; each
/// element is a 2D/3D vector to be transformed.
/// * dst: output array of the same size and type as src.
/// * m: 3x3 or 4x4 floating-point transformation matrix.
/// ## See also
/// transform, warpPerspective, getPerspectiveTransform, findHomography
pub fn perspective_transform(src: &core::Mat, dst: &mut core::Mat, m: &core::Mat) -> Result<()> {
    unsafe { sys::cv_perspectiveTransform_Mat_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat(), m.as_raw_Mat()) }.into_result()
}

/// Calculates the rotation angle of 2D vectors.
/// 
/// The function cv::phase calculates the rotation angle of each 2D vector that
/// is formed from the corresponding elements of x and y :
/// <div lang='latex'>\texttt{angle} (I) =  \texttt{atan2} ( \texttt{y} (I), \texttt{x} (I))</div>
/// 
/// The angle estimation accuracy is about 0.3 degrees. When x(I)=y(I)=0 ,
/// the corresponding angle(I) is set to 0.
/// ## Parameters
/// * x: input floating-point array of x-coordinates of 2D vectors.
/// * y: input array of y-coordinates of 2D vectors; it must have the
/// same size and the same type as x.
/// * angle: output array of vector angles; it has the same size and
/// same type as x .
/// * angleInDegrees: when true, the function calculates the angle in
/// degrees, otherwise, they are measured in radians.
///
/// ## C++ default parameters
/// * angle_in_degrees: false
pub fn phase(x: &core::Mat, y: &core::Mat, angle: &mut core::Mat, angle_in_degrees: bool) -> Result<()> {
    unsafe { sys::cv_phase_Mat_Mat_Mat_bool(x.as_raw_Mat(), y.as_raw_Mat(), angle.as_raw_Mat(), angle_in_degrees) }.into_result()
}

/// Calculates x and y coordinates of 2D vectors from their magnitude and angle.
/// 
/// The function cv::polarToCart calculates the Cartesian coordinates of each 2D
/// vector represented by the corresponding elements of magnitude and angle:
/// <div lang='latex'>\begin{array}{l} \texttt{x} (I) =  \texttt{magnitude} (I) \cos ( \texttt{angle} (I)) \\ \texttt{y} (I) =  \texttt{magnitude} (I) \sin ( \texttt{angle} (I)) \\ \end{array}</div>
/// 
/// The relative accuracy of the estimated coordinates is about 1e-6.
/// ## Parameters
/// * magnitude: input floating-point array of magnitudes of 2D vectors;
/// it can be an empty matrix (=Mat()), in this case, the function assumes
/// that all the magnitudes are =1; if it is not empty, it must have the
/// same size and type as angle.
/// * angle: input floating-point array of angles of 2D vectors.
/// * x: output array of x-coordinates of 2D vectors; it has the same
/// size and type as angle.
/// * y: output array of y-coordinates of 2D vectors; it has the same
/// size and type as angle.
/// * angleInDegrees: when true, the input angles are measured in
/// degrees, otherwise, they are measured in radians.
/// ## See also
/// cartToPolar, magnitude, phase, exp, log, pow, sqrt
///
/// ## C++ default parameters
/// * angle_in_degrees: false
pub fn polar_to_cart(magnitude: &core::Mat, angle: &core::Mat, x: &mut core::Mat, y: &mut core::Mat, angle_in_degrees: bool) -> Result<()> {
    unsafe { sys::cv_polarToCart_Mat_Mat_Mat_Mat_bool(magnitude.as_raw_Mat(), angle.as_raw_Mat(), x.as_raw_Mat(), y.as_raw_Mat(), angle_in_degrees) }.into_result()
}

/// Raises every array element to a power.
/// 
/// The function cv::pow raises every element of the input array to power :
/// <div lang='latex'>\texttt{dst} (I) =  \fork{\texttt{src}(I)^{power}}{if \(\texttt{power}\) is integer}{|\texttt{src}(I)|^{power}}{otherwise}</div>
/// 
/// So, for a non-integer power exponent, the absolute values of input array
/// elements are used. However, it is possible to get true values for
/// negative values using some extra operations. In the example below,
/// computing the 5th root of array src shows:
/// ```ignore{.cpp}
/// Mat mask = src < 0;
/// pow(src, 1./5, dst);
/// subtract(Scalar::all(0), dst, dst, mask);
/// ```
/// 
/// For some values of power, such as integer values, 0.5 and -0.5,
/// specialized faster algorithms are used.
/// 
/// Special values (NaN, Inf) are not handled.
/// ## Parameters
/// * src: input array.
/// * power: exponent of power.
/// * dst: output array of the same size and type as src.
/// ## See also
/// sqrt, exp, log, cartToPolar, polarToCart
pub fn pow(src: &core::Mat, power: f64, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_pow_Mat_double_Mat(src.as_raw_Mat(), power, dst.as_raw_Mat()) }.into_result()
}

/// Fills the array with normally distributed random numbers.
/// 
/// The function cv::randn fills the matrix dst with normally distributed random numbers with the specified
/// mean vector and the standard deviation matrix. The generated random numbers are clipped to fit the
/// value range of the output array data type.
/// ## Parameters
/// * dst: output array of random numbers; the array must be pre-allocated and have 1 to 4 channels.
/// * mean: mean value (expectation) of the generated random numbers.
/// * stddev: standard deviation of the generated random numbers; it can be either a vector (in
/// which case a diagonal standard deviation matrix is assumed) or a square matrix.
/// ## See also
/// RNG, randu
pub fn randn(dst: &mut core::Mat, mean: &core::Mat, stddev: &core::Mat) -> Result<()> {
    unsafe { sys::cv_randn_Mat_Mat_Mat(dst.as_raw_Mat(), mean.as_raw_Mat(), stddev.as_raw_Mat()) }.into_result()
}

/// Generates a single uniformly-distributed random number or an array of random numbers.
/// 
/// Non-template variant of the function fills the matrix dst with uniformly-distributed
/// random numbers from the specified range:
/// <div lang='latex'>\texttt{low} _c  \leq \texttt{dst} (I)_c <  \texttt{high} _c</div>
/// ## Parameters
/// * dst: output array of random numbers; the array must be pre-allocated.
/// * low: inclusive lower boundary of the generated random numbers.
/// * high: exclusive upper boundary of the generated random numbers.
/// ## See also
/// RNG, randn, theRNG
pub fn randu(dst: &mut core::Mat, low: &core::Mat, high: &core::Mat) -> Result<()> {
    unsafe { sys::cv_randu_Mat_Mat_Mat(dst.as_raw_Mat(), low.as_raw_Mat(), high.as_raw_Mat()) }.into_result()
}

/// Reduces a matrix to a vector.
/// 
/// The function #reduce reduces the matrix to a vector by treating the matrix rows/columns as a set of
/// 1D vectors and performing the specified operation on the vectors until a single row/column is
/// obtained. For example, the function can be used to compute horizontal and vertical projections of a
/// raster image. In case of #REDUCE_MAX and #REDUCE_MIN , the output image should have the same type as the source one.
/// In case of #REDUCE_SUM and #REDUCE_AVG , the output may have a larger element bit-depth to preserve accuracy.
/// And multi-channel arrays are also supported in these two reduction modes.
/// 
/// The following code demonstrates its usage for a single channel matrix.
/// @snippet snippets/core_reduce.cpp example
/// 
/// And the following code demonstrates its usage for a two-channel matrix.
/// @snippet snippets/core_reduce.cpp example2
/// 
/// ## Parameters
/// * src: input 2D matrix.
/// * dst: output vector. Its size and type is defined by dim and dtype parameters.
/// * dim: dimension index along which the matrix is reduced. 0 means that the matrix is reduced to
/// a single row. 1 means that the matrix is reduced to a single column.
/// * rtype: reduction operation that could be one of #ReduceTypes
/// * dtype: when negative, the output vector will have the same type as the input matrix,
/// otherwise, its type will be CV_MAKE_TYPE(CV_MAT_DEPTH(dtype), src.channels()).
/// ## See also
/// repeat
///
/// ## C++ default parameters
/// * dtype: -1
pub fn reduce(src: &core::Mat, dst: &mut core::Mat, dim: i32, rtype: i32, dtype: i32) -> Result<()> {
    unsafe { sys::cv_reduce_Mat_Mat_int_int_int(src.as_raw_Mat(), dst.as_raw_Mat(), dim, rtype, dtype) }.into_result()
}

/// Fills the output array with repeated copies of the input array.
/// 
/// The function cv::repeat duplicates the input array one or more times along each of the two axes:
/// <div lang='latex'>\texttt{dst} _{ij}= \texttt{src} _{i\mod src.rows, \; j\mod src.cols }</div>
/// The second variant of the function is more convenient to use with @ref MatrixExpressions.
/// ## Parameters
/// * src: input array to replicate.
/// * ny: Flag to specify how many times the `src` is repeated along the
/// vertical axis.
/// * nx: Flag to specify how many times the `src` is repeated along the
/// horizontal axis.
/// * dst: output array of the same type as `src`.
/// ## See also
/// cv::reduce
/// 
/// ## Overloaded parameters
/// 
/// * src: input array to replicate.
/// * ny: Flag to specify how many times the `src` is repeated along the
/// vertical axis.
/// * nx: Flag to specify how many times the `src` is repeated along the
/// horizontal axis.
pub fn repeat(src: &core::Mat, ny: i32, nx: i32) -> Result<core::Mat> {
    unsafe { sys::cv_repeat_Mat_int_int(src.as_raw_Mat(), ny, nx) }.into_result().map(|x| core::Mat { ptr: x })
}

/// Fills the output array with repeated copies of the input array.
/// 
/// The function cv::repeat duplicates the input array one or more times along each of the two axes:
/// <div lang='latex'>\texttt{dst} _{ij}= \texttt{src} _{i\mod src.rows, \; j\mod src.cols }</div>
/// The second variant of the function is more convenient to use with @ref MatrixExpressions.
/// ## Parameters
/// * src: input array to replicate.
/// * ny: Flag to specify how many times the `src` is repeated along the
/// vertical axis.
/// * nx: Flag to specify how many times the `src` is repeated along the
/// horizontal axis.
/// * dst: output array of the same type as `src`.
/// ## See also
/// cv::reduce
pub fn repeat_to(src: &core::Mat, ny: i32, nx: i32, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_repeat_Mat_int_int_Mat(src.as_raw_Mat(), ny, nx, dst.as_raw_Mat()) }.into_result()
}

/// Rotates a 2D array in multiples of 90 degrees.
/// The function cv::rotate rotates the array in one of three different ways:
///   Rotate by 90 degrees clockwise (rotateCode = ROTATE_90_CLOCKWISE).
///   Rotate by 180 degrees clockwise (rotateCode = ROTATE_180).
///   Rotate by 270 degrees clockwise (rotateCode = ROTATE_90_COUNTERCLOCKWISE).
/// ## Parameters
/// * src: input array.
/// * dst: output array of the same type as src.  The size is the same with ROTATE_180,
/// and the rows and cols are switched for ROTATE_90_CLOCKWISE and ROTATE_90_COUNTERCLOCKWISE.
/// * rotateCode: an enum to specify how to rotate the array; see the enum #RotateFlags
/// ## See also
/// transpose , repeat , completeSymm, flip, RotateFlags
pub fn rotate(src: &core::Mat, dst: &mut core::Mat, rotate_code: i32) -> Result<()> {
    unsafe { sys::cv_rotate_Mat_Mat_int(src.as_raw_Mat(), dst.as_raw_Mat(), rotate_code) }.into_result()
}

/// Round first value up to the nearest multiple of second value.
/// 
/// Use this function instead of `ceil((float)a / b) * b` expressions.
/// 
/// ## See also
/// divUp
pub fn round_up(a: i32, b: u32) -> Result<i32> {
    unsafe { sys::cv_roundUp_int_unsigned_int(a, b) }.into_result()
}

/// Round first value up to the nearest multiple of second value.
/// 
/// Use this function instead of `ceil((float)a / b) * b` expressions.
/// 
/// ## See also
/// divUp
/// 
/// ## Overloaded parameters
pub fn round_up_1(a: size_t, b: u32) -> Result<size_t> {
    unsafe { sys::cv_roundUp_size_t_unsigned_int(a, b) }.into_result()
}

/// Override search data path by adding new search location
/// 
/// Use this only to override default behavior
/// Passed paths are used in LIFO order.
/// 
/// ## Parameters
/// * path: Path to used samples data
pub fn add_samples_data_search_path(path: &str) -> Result<()> {
    string_arg!(path);
    unsafe { sys::cv_samples_addSamplesDataSearchPath_String(path.as_ptr()) }.into_result()
}

/// Append samples search data sub directory
/// 
/// General usage is to add OpenCV modules name (`<opencv_contrib>/modules/<name>/samples/data` -> `<name>/samples/data` + `modules/<name>/samples/data`).
/// Passed subdirectories are used in LIFO order.
/// 
/// ## Parameters
/// * subdir: samples data sub directory
pub fn add_samples_data_search_sub_directory(subdir: &str) -> Result<()> {
    string_arg!(subdir);
    unsafe { sys::cv_samples_addSamplesDataSearchSubDirectory_String(subdir.as_ptr()) }.into_result()
}

///
/// ## C++ default parameters
/// * silent_mode: false
pub fn find_file_or_keep(relative_path: &str, silent_mode: bool) -> Result<String> {
    string_arg!(relative_path);
    unsafe { sys::cv_samples_findFileOrKeep_String_bool(relative_path.as_ptr(), silent_mode) }.into_result().map(crate::templ::receive_string_mut)
}

/// Try to find requested data file
/// 
/// Search directories:
/// 
/// 1. Directories passed via `addSamplesDataSearchPath()`
/// 2. OPENCV_SAMPLES_DATA_PATH_HINT environment variable
/// 3. OPENCV_SAMPLES_DATA_PATH environment variable
/// If parameter value is not empty and nothing is found then stop searching.
/// 4. Detects build/install path based on:
/// a. current working directory (CWD)
/// b. and/or binary module location (opencv_core/opencv_world, doesn't work with static linkage)
/// 5. Scan `<source>/{,data,samples/data}` directories if build directory is detected or the current directory is in source tree.
/// 6. Scan `<install>/share/OpenCV` directory if install directory is detected.
/// 
/// @see cv::utils::findDataFile
/// 
/// ## Parameters
/// * relative_path: Relative path to data file
/// * required: Specify "file not found" handling.
/// If true, function prints information message and raises cv::Exception.
/// If false, function returns empty result
/// * silentMode: Disables messages
/// ## Returns
/// Returns path (absolute or relative to the current directory) or empty string if file is not found
///
/// ## C++ default parameters
/// * required: true
/// * silent_mode: false
pub fn find_file(relative_path: &str, required: bool, silent_mode: bool) -> Result<String> {
    string_arg!(relative_path);
    unsafe { sys::cv_samples_findFile_String_bool_bool(relative_path.as_ptr(), required, silent_mode) }.into_result().map(crate::templ::receive_string_mut)
}

/// Calculates the sum of a scaled array and another array.
/// 
/// The function scaleAdd is one of the classical primitive linear algebra operations, known as DAXPY
/// or SAXPY in [BLAS](http://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms). It calculates
/// the sum of a scaled array and another array:
/// <div lang='latex'>\texttt{dst} (I)= \texttt{scale} \cdot \texttt{src1} (I) +  \texttt{src2} (I)</div>
/// The function can also be emulated with a matrix expression, for example:
/// ```ignore{.cpp}
/// Mat A(3, 3, CV_64F);
/// ...
/// A.row(0) = A.row(1)*2 + A.row(2);
/// ```
/// 
/// ## Parameters
/// * src1: first input array.
/// * alpha: scale factor for the first array.
/// * src2: second input array of the same size and type as src1.
/// * dst: output array of the same size and type as src1.
/// ## See also
/// add, addWeighted, subtract, Mat::dot, Mat::convertTo
pub fn scale_add(src1: &core::Mat, alpha: f64, src2: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_scaleAdd_Mat_double_Mat_Mat(src1.as_raw_Mat(), alpha, src2.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Sets/resets the break-on-error mode.
/// 
/// When the break-on-error mode is set, the default error handler issues a hardware exception, which
/// can make debugging more convenient.
/// 
/// \return the previous state
pub fn set_break_on_error(flag: bool) -> Result<bool> {
    unsafe { sys::cv_setBreakOnError_bool(flag) }.into_result()
}

/// Initializes a scaled identity matrix.
/// 
/// The function cv::setIdentity initializes a scaled identity matrix:
/// <div lang='latex'>\texttt{mtx} (i,j)= \fork{\texttt{value}}{ if \(i=j\)}{0}{otherwise}</div>
/// 
/// The function can also be emulated using the matrix initializers and the
/// matrix expressions:
/// ```ignore
/// Mat A = Mat::eye(4, 3, CV_32F)*5;
/// // A will be set to [[5, 0, 0], [0, 5, 0], [0, 0, 5], [0, 0, 0]]
/// ```
/// 
/// ## Parameters
/// * mtx: matrix to initialize (not necessarily square).
/// * s: value to assign to diagonal elements.
/// ## See also
/// Mat::zeros, Mat::ones, Mat::setTo, Mat::operator=
///
/// ## C++ default parameters
/// * s: Scalar(1)
pub fn set_identity(mtx: &mut core::Mat, s: core::Scalar) -> Result<()> {
    unsafe { sys::cv_setIdentity_Mat_Scalar(mtx.as_raw_Mat(), s) }.into_result()
}

/// OpenCV will try to set the number of threads for the next parallel region.
/// 
/// If threads == 0, OpenCV will disable threading optimizations and run all it's functions
/// sequentially. Passing threads \< 0 will reset threads number to system default. This function must
/// be called outside of parallel region.
/// 
/// OpenCV will try to run its functions with specified threads number, but some behaviour differs from
/// framework:
/// *   `TBB` - User-defined parallel constructions will run with the same threads number, if
/// another is not specified. If later on user creates his own scheduler, OpenCV will use it.
/// *   `OpenMP` - No special defined behaviour.
/// *   `Concurrency` - If threads == 1, OpenCV will disable threading optimizations and run its
/// functions sequentially.
/// *   `GCD` - Supports only values \<= 0.
/// *   `C=` - No special defined behaviour.
/// ## Parameters
/// * nthreads: Number of threads used by OpenCV.
/// ## See also
/// getNumThreads, getThreadNum
pub fn set_num_threads(nthreads: i32) -> Result<()> {
    unsafe { sys::cv_setNumThreads_int(nthreads) }.into_result()
}

/// Sets state of default random number generator.
/// 
/// The function cv::setRNGSeed sets state of default random number generator to custom value.
/// ## Parameters
/// * seed: new state for default random number generator
/// ## See also
/// RNG, randu, randn
pub fn set_rng_seed(seed: i32) -> Result<()> {
    unsafe { sys::cv_setRNGSeed_int(seed) }.into_result()
}

pub fn set_use_open_vx(flag: bool) -> Result<()> {
    unsafe { sys::cv_setUseOpenVX_bool(flag) }.into_result()
}

/// Enables or disables the optimized code.
/// 
/// The function can be used to dynamically turn on and off optimized dispatched code (code that uses SSE4.2, AVX/AVX2,
/// and other instructions on the platforms that support it). It sets a global flag that is further
/// checked by OpenCV functions. Since the flag is not checked in the inner OpenCV loops, it is only
/// safe to call the function on the very top level in your application where you can be sure that no
/// other OpenCV function is currently executed.
/// 
/// By default, the optimized code is enabled unless you disable it in CMake. The current status can be
/// retrieved using useOptimized.
/// ## Parameters
/// * onoff: The boolean flag specifying whether the optimized code should be used (onoff=true)
/// or not (onoff=false).
pub fn set_use_optimized(onoff: bool) -> Result<()> {
    unsafe { sys::cv_setUseOptimized_bool(onoff) }.into_result()
}

/// Finds the real roots of a cubic equation.
/// 
/// The function solveCubic finds the real roots of a cubic equation:
/// *   if coeffs is a 4-element vector:
/// <div lang='latex'>\texttt{coeffs} [0] x^3 +  \texttt{coeffs} [1] x^2 +  \texttt{coeffs} [2] x +  \texttt{coeffs} [3] = 0</div>
/// *   if coeffs is a 3-element vector:
/// <div lang='latex'>x^3 +  \texttt{coeffs} [0] x^2 +  \texttt{coeffs} [1] x +  \texttt{coeffs} [2] = 0</div>
/// 
/// The roots are stored in the roots array.
/// ## Parameters
/// * coeffs: equation coefficients, an array of 3 or 4 elements.
/// * roots: output array of real roots that has 1 or 3 elements.
/// ## Returns
/// number of real roots. It can be 0, 1 or 2.
pub fn solve_cubic(coeffs: &core::Mat, roots: &mut core::Mat) -> Result<i32> {
    unsafe { sys::cv_solveCubic_Mat_Mat(coeffs.as_raw_Mat(), roots.as_raw_Mat()) }.into_result()
}

/// Solve given (non-integer) linear programming problem using the Simplex Algorithm (Simplex Method).
/// 
/// What we mean here by "linear programming problem" (or LP problem, for short) can be formulated as:
/// 
/// <div lang='latex'>\mbox{Maximize } c\cdot x\\
/// \mbox{Subject to:}\\
/// Ax\leq b\\
/// x\geq 0</div>
/// 
/// Where <span lang='latex'>c</span> is fixed `1`-by-`n` row-vector, <span lang='latex'>A</span> is fixed `m`-by-`n` matrix, <span lang='latex'>b</span> is fixed `m`-by-`1`
/// column vector and <span lang='latex'>x</span> is an arbitrary `n`-by-`1` column vector, which satisfies the constraints.
/// 
/// Simplex algorithm is one of many algorithms that are designed to handle this sort of problems
/// efficiently. Although it is not optimal in theoretical sense (there exist algorithms that can solve
/// any problem written as above in polynomial time, while simplex method degenerates to exponential
/// time for some special cases), it is well-studied, easy to implement and is shown to work well for
/// real-life purposes.
/// 
/// The particular implementation is taken almost verbatim from **Introduction to Algorithms, third
/// edition** by T. H. Cormen, C. E. Leiserson, R. L. Rivest and Clifford Stein. In particular, the
/// Bland's rule <http://en.wikipedia.org/wiki/Bland%27s_rule> is used to prevent cycling.
/// 
/// ## Parameters
/// * Func: This row-vector corresponds to <span lang='latex'>c</span> in the LP problem formulation (see above). It should
/// contain 32- or 64-bit floating point numbers. As a convenience, column-vector may be also submitted,
/// in the latter case it is understood to correspond to <span lang='latex'>c^T</span>.
/// * Constr: `m`-by-`n+1` matrix, whose rightmost column corresponds to <span lang='latex'>b</span> in formulation above
/// and the remaining to <span lang='latex'>A</span>. It should contain 32- or 64-bit floating point numbers.
/// * z: The solution will be returned here as a column-vector - it corresponds to <span lang='latex'>c</span> in the
/// formulation above. It will contain 64-bit floating point numbers.
/// ## Returns
/// One of cv::SolveLPResult
pub fn solve_lp(func: &core::Mat, constr: &core::Mat, z: &mut core::Mat) -> Result<i32> {
    unsafe { sys::cv_solveLP_Mat_Mat_Mat(func.as_raw_Mat(), constr.as_raw_Mat(), z.as_raw_Mat()) }.into_result()
}

/// Finds the real or complex roots of a polynomial equation.
/// 
/// The function cv::solvePoly finds real and complex roots of a polynomial equation:
/// <div lang='latex'>\texttt{coeffs} [n] x^{n} +  \texttt{coeffs} [n-1] x^{n-1} + ... +  \texttt{coeffs} [1] x +  \texttt{coeffs} [0] = 0</div>
/// ## Parameters
/// * coeffs: array of polynomial coefficients.
/// * roots: output (complex) array of roots.
/// * maxIters: maximum number of iterations the algorithm does.
///
/// ## C++ default parameters
/// * max_iters: 300
pub fn solve_poly(coeffs: &core::Mat, roots: &mut core::Mat, max_iters: i32) -> Result<f64> {
    unsafe { sys::cv_solvePoly_Mat_Mat_int(coeffs.as_raw_Mat(), roots.as_raw_Mat(), max_iters) }.into_result()
}

/// Solves one or more linear systems or least-squares problems.
/// 
/// The function cv::solve solves a linear system or least-squares problem (the
/// latter is possible with SVD or QR methods, or by specifying the flag
/// #DECOMP_NORMAL ):
/// <div lang='latex'>\texttt{dst} =  \arg \min _X \| \texttt{src1} \cdot \texttt{X} -  \texttt{src2} \|</div>
/// 
/// If #DECOMP_LU or #DECOMP_CHOLESKY method is used, the function returns 1
/// if src1 (or <span lang='latex'>\texttt{src1}^T\texttt{src1}</span> ) is non-singular. Otherwise,
/// it returns 0. In the latter case, dst is not valid. Other methods find a
/// pseudo-solution in case of a singular left-hand side part.
/// 
/// 
/// Note: If you want to find a unity-norm solution of an under-defined
/// singular system <span lang='latex'>\texttt{src1}\cdot\texttt{dst}=0</span> , the function solve
/// will not do the work. Use SVD::solveZ instead.
/// 
/// ## Parameters
/// * src1: input matrix on the left-hand side of the system.
/// * src2: input matrix on the right-hand side of the system.
/// * dst: output solution.
/// * flags: solution (matrix inversion) method (#DecompTypes)
/// ## See also
/// invert, SVD, eigen
///
/// ## C++ default parameters
/// * flags: DECOMP_LU
pub fn solve(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat, flags: i32) -> Result<bool> {
    unsafe { sys::cv_solve_Mat_Mat_Mat_int(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat(), flags) }.into_result()
}

/// Sorts each row or each column of a matrix.
/// 
/// The function cv::sortIdx sorts each matrix row or each matrix column in the
/// ascending or descending order. So you should pass two operation flags to
/// get desired behaviour. Instead of reordering the elements themselves, it
/// stores the indices of sorted elements in the output array. For example:
/// ```ignore
/// Mat A = Mat::eye(3,3,CV_32F), B;
/// sortIdx(A, B, SORT_EVERY_ROW + SORT_ASCENDING);
/// // B will probably contain
/// // (because of equal elements in A some permutations are possible):
/// // [[1, 2, 0], [0, 2, 1], [0, 1, 2]]
/// ```
/// 
/// ## Parameters
/// * src: input single-channel array.
/// * dst: output integer array of the same size as src.
/// * flags: operation flags that could be a combination of cv::SortFlags
/// ## See also
/// sort, randShuffle
pub fn sort_idx(src: &core::Mat, dst: &mut core::Mat, flags: i32) -> Result<()> {
    unsafe { sys::cv_sortIdx_Mat_Mat_int(src.as_raw_Mat(), dst.as_raw_Mat(), flags) }.into_result()
}

/// Sorts each row or each column of a matrix.
/// 
/// The function cv::sort sorts each matrix row or each matrix column in
/// ascending or descending order. So you should pass two operation flags to
/// get desired behaviour. If you want to sort matrix rows or columns
/// lexicographically, you can use STL std::sort generic function with the
/// proper comparison predicate.
/// 
/// ## Parameters
/// * src: input single-channel array.
/// * dst: output array of the same size and type as src.
/// * flags: operation flags, a combination of #SortFlags
/// ## See also
/// sortIdx, randShuffle
pub fn sort(src: &core::Mat, dst: &mut core::Mat, flags: i32) -> Result<()> {
    unsafe { sys::cv_sort_Mat_Mat_int(src.as_raw_Mat(), dst.as_raw_Mat(), flags) }.into_result()
}

/// Divides a multi-channel array into several single-channel arrays.
/// 
/// The function cv::split splits a multi-channel array into separate single-channel arrays:
/// <div lang='latex'>\texttt{mv} [c](I) =  \texttt{src} (I)_c</div>
/// If you need to extract a single channel or do some other sophisticated channel permutation, use
/// mixChannels .
/// 
/// The following example demonstrates how to split a 3-channel matrix into 3 single channel matrices.
/// @snippet snippets/core_split.cpp example
/// 
/// ## Parameters
/// * src: input multi-channel array.
/// * mvbegin: output array; the number of arrays must match src.channels(); the arrays themselves are
/// reallocated, if needed.
/// ## See also
/// merge, mixChannels, cvtColor
/// 
/// ## Overloaded parameters
/// 
/// * m: input multi-channel array.
/// * mv: output vector of arrays; the arrays themselves are reallocated, if needed.
pub fn split(m: &core::Mat, mv: &mut types::VectorOfMat) -> Result<()> {
    unsafe { sys::cv_split_Mat_VectorOfMat(m.as_raw_Mat(), mv.as_raw_VectorOfMat()) }.into_result()
}

/// Calculates a square root of array elements.
/// 
/// The function cv::sqrt calculates a square root of each input array element.
/// In case of multi-channel arrays, each channel is processed
/// independently. The accuracy is approximately the same as of the built-in
/// std::sqrt .
/// ## Parameters
/// * src: input floating-point array.
/// * dst: output array of the same size and type as src.
pub fn sqrt(src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_sqrt_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Calculates the per-element difference between two arrays or array and a scalar.
/// 
/// The function subtract calculates:
/// - Difference between two arrays, when both input arrays have the same size and the same number of
/// channels:
/// <div lang='latex'>\texttt{dst}(I) =  \texttt{saturate} ( \texttt{src1}(I) -  \texttt{src2}(I)) \quad \texttt{if mask}(I) \ne0</div>
/// - Difference between an array and a scalar, when src2 is constructed from Scalar or has the same
/// number of elements as `src1.channels()`:
/// <div lang='latex'>\texttt{dst}(I) =  \texttt{saturate} ( \texttt{src1}(I) -  \texttt{src2} ) \quad \texttt{if mask}(I) \ne0</div>
/// - Difference between a scalar and an array, when src1 is constructed from Scalar or has the same
/// number of elements as `src2.channels()`:
/// <div lang='latex'>\texttt{dst}(I) =  \texttt{saturate} ( \texttt{src1} -  \texttt{src2}(I) ) \quad \texttt{if mask}(I) \ne0</div>
/// - The reverse difference between a scalar and an array in the case of `SubRS`:
/// <div lang='latex'>\texttt{dst}(I) =  \texttt{saturate} ( \texttt{src2} -  \texttt{src1}(I) ) \quad \texttt{if mask}(I) \ne0</div>
/// where I is a multi-dimensional index of array elements. In case of multi-channel arrays, each
/// channel is processed independently.
/// 
/// The first function in the list above can be replaced with matrix expressions:
/// ```ignore{.cpp}
/// dst = src1 - src2;
/// dst -= src1; // equivalent to subtract(dst, src1, dst);
/// ```
/// 
/// The input arrays and the output array can all have the same or different depths. For example, you
/// can subtract to 8-bit unsigned arrays and store the difference in a 16-bit signed array. Depth of
/// the output array is determined by dtype parameter. In the second and third cases above, as well as
/// in the first case, when src1.depth() == src2.depth(), dtype can be set to the default -1. In this
/// case the output array will have the same depth as the input array, be it src1, src2 or both.
/// 
/// Note: Saturation is not applied when the output array has the depth CV_32S. You may even get
/// result of an incorrect sign in the case of overflow.
/// ## Parameters
/// * src1: first input array or a scalar.
/// * src2: second input array or a scalar.
/// * dst: output array of the same size and the same number of channels as the input array.
/// * mask: optional operation mask; this is an 8-bit single channel array that specifies elements
/// of the output array to be changed.
/// * dtype: optional depth of the output array
/// ## See also
/// add, addWeighted, scaleAdd, Mat::convertTo
///
/// ## C++ default parameters
/// * mask: noArray()
/// * dtype: -1
pub fn subtract(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat, mask: &core::Mat, dtype: i32) -> Result<()> {
    unsafe { sys::cv_subtract_Mat_Mat_Mat_Mat_int(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat(), mask.as_raw_Mat(), dtype) }.into_result()
}

/// Calculates the sum of array elements.
/// 
/// The function cv::sum calculates and returns the sum of array elements,
/// independently for each channel.
/// ## Parameters
/// * src: input array that must have from 1 to 4 channels.
/// ## See also
/// countNonZero, mean, meanStdDev, norm, minMaxLoc, reduce
pub fn sum(src: &core::Mat) -> Result<core::Scalar> {
    unsafe { sys::cv_sum_Mat(src.as_raw_Mat()) }.into_result()
}

/// Swaps two matrices
pub fn swap(a: &mut core::Mat, b: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_swap_Mat_Mat(a.as_raw_Mat(), b.as_raw_Mat()) }.into_result()
}

///
/// ## C++ default parameters
/// * suffix: 0
pub fn tempfile(suffix: &str) -> Result<String> {
    string_arg!(suffix);
    unsafe { sys::cv_tempfile_const_char_X(suffix.as_ptr()) }.into_result().map(crate::templ::receive_string_mut)
}

/// Returns the trace of a matrix.
/// 
/// The function cv::trace returns the sum of the diagonal elements of the
/// matrix mtx .
/// <div lang='latex'>\mathrm{tr} ( \texttt{mtx} ) =  \sum _i  \texttt{mtx} (i,i)</div>
/// ## Parameters
/// * mtx: input matrix.
pub fn trace(mtx: &core::Mat) -> Result<core::Scalar> {
    unsafe { sys::cv_trace_Mat(mtx.as_raw_Mat()) }.into_result()
}

/// Performs the matrix transformation of every array element.
/// 
/// The function cv::transform performs the matrix transformation of every
/// element of the array src and stores the results in dst :
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{m} \cdot \texttt{src} (I)</div>
/// (when m.cols=src.channels() ), or
/// <div lang='latex'>\texttt{dst} (I) =  \texttt{m} \cdot [ \texttt{src} (I); 1]</div>
/// (when m.cols=src.channels()+1 )
/// 
/// Every element of the N -channel array src is interpreted as N -element
/// vector that is transformed using the M x N or M x (N+1) matrix m to
/// M-element vector - the corresponding element of the output array dst .
/// 
/// The function may be used for geometrical transformation of
/// N -dimensional points, arbitrary linear color space transformation (such
/// as various kinds of RGB to YUV transforms), shuffling the image
/// channels, and so forth.
/// ## Parameters
/// * src: input array that must have as many channels (1 to 4) as
/// m.cols or m.cols-1.
/// * dst: output array of the same size and depth as src; it has as
/// many channels as m.rows.
/// * m: transformation 2x2 or 2x3 floating-point matrix.
/// ## See also
/// perspectiveTransform, getAffineTransform, estimateAffine2D, warpAffine, warpPerspective
pub fn transform(src: &core::Mat, dst: &mut core::Mat, m: &core::Mat) -> Result<()> {
    unsafe { sys::cv_transform_Mat_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat(), m.as_raw_Mat()) }.into_result()
}

/// Transposes a matrix.
/// 
/// The function cv::transpose transposes the matrix src :
/// <div lang='latex'>\texttt{dst} (i,j) =  \texttt{src} (j,i)</div>
/// 
/// Note: No complex conjugation is done in case of a complex matrix. It
/// should be done separately if needed.
/// ## Parameters
/// * src: input array.
/// * dst: output array of the same type as src.
pub fn transpose(src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_transpose_Mat_Mat(src.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Returns string of cv::Mat depth value: CV_8UC3 -> "CV_8UC3" or "<invalid type>"
pub fn type_to_string(_type: i32) -> Result<String> {
    unsafe { sys::cv_typeToString_int(_type) }.into_result().map(crate::templ::receive_string)
}

pub fn use_open_vx() -> Result<bool> {
    unsafe { sys::cv_useOpenVX() }.into_result()
}

/// Returns the status of optimized code usage.
/// 
/// The function returns true if the optimized code is enabled. Otherwise, it returns false.
pub fn use_optimized() -> Result<bool> {
    unsafe { sys::cv_useOptimized() }.into_result()
}

pub fn dump_input_array_of_arrays(argument: &types::VectorOfMat) -> Result<String> {
    unsafe { sys::cv_utils_dumpInputArrayOfArrays_VectorOfMat(argument.as_raw_VectorOfMat()) }.into_result().map(crate::templ::receive_string_mut)
}

pub fn dump_input_array(argument: &core::Mat) -> Result<String> {
    unsafe { sys::cv_utils_dumpInputArray_Mat(argument.as_raw_Mat()) }.into_result().map(crate::templ::receive_string_mut)
}

pub fn dump_input_output_array_of_arrays(argument: &mut types::VectorOfMat) -> Result<String> {
    unsafe { sys::cv_utils_dumpInputOutputArrayOfArrays_VectorOfMat(argument.as_raw_VectorOfMat()) }.into_result().map(crate::templ::receive_string_mut)
}

pub fn dump_input_output_array(argument: &mut core::Mat) -> Result<String> {
    unsafe { sys::cv_utils_dumpInputOutputArray_Mat(argument.as_raw_Mat()) }.into_result().map(crate::templ::receive_string_mut)
}

pub fn get_thread_id() -> Result<i32> {
    unsafe { sys::cv_utils_getThreadID() }.into_result()
}

/// Converts VASurfaceID object to OutputArray.
/// ## Parameters
/// * display: - VADisplay object.
/// * surface: - source VASurfaceID object.
/// * size: - size of image represented by VASurfaceID object.
/// * dst: - destination OutputArray.
pub fn convert_from_va_surface(display: &mut c_void, surface: u32, size: core::Size, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_va_intel_convertFromVASurface_void_X_unsigned_int_Size_Mat(display, surface, size, dst.as_raw_Mat()) }.into_result()
}

/// Converts InputArray to VASurfaceID object.
/// ## Parameters
/// * display: - VADisplay object.
/// * src: - source InputArray.
/// * surface: - destination VASurfaceID object.
/// * size: - size of image represented by VASurfaceID object.
pub fn convert_to_va_surface(display: &mut c_void, src: &core::Mat, surface: u32, size: core::Size) -> Result<()> {
    unsafe { sys::cv_va_intel_convertToVASurface_void_X_Mat_unsigned_int_Size(display, src.as_raw_Mat(), surface, size) }.into_result()
}

/// Applies vertical concatenation to given matrices.
/// 
/// The function vertically concatenates two or more cv::Mat matrices (with the same number of cols).
/// ```ignore{.cpp}
/// cv::Mat matArray[] = { cv::Mat(1, 4, CV_8UC1, cv::Scalar(1)),
/// cv::Mat(1, 4, CV_8UC1, cv::Scalar(2)),
/// cv::Mat(1, 4, CV_8UC1, cv::Scalar(3)),};
/// 
/// cv::Mat out;
/// cv::vconcat( matArray, 3, out );
/// //out:
/// //[1,   1,   1,   1;
/// // 2,   2,   2,   2;
/// // 3,   3,   3,   3]
/// ```
/// 
/// ## Parameters
/// * src: input array or vector of matrices. all of the matrices must have the same number of cols and the same depth.
/// * nsrc: number of matrices in src.
/// * dst: output array. It has the same number of cols and depth as the src, and the sum of rows of the src.
/// ## See also
/// cv::hconcat(const Mat*, size_t, OutputArray),  cv::hconcat(InputArrayOfArrays, OutputArray) and  cv::hconcat(InputArray, InputArray, OutputArray)
/// 
/// ## Overloaded parameters
/// 
/// ```ignore{.cpp}
/// cv::Mat_<float> A = (cv::Mat_<float>(3, 2) << 1, 7,
/// 2, 8,
/// 3, 9);
/// cv::Mat_<float> B = (cv::Mat_<float>(3, 2) << 4, 10,
/// 5, 11,
/// 6, 12);
/// 
/// cv::Mat C;
/// cv::vconcat(A, B, C);
/// //C:
/// //[1, 7;
/// // 2, 8;
/// // 3, 9;
/// // 4, 10;
/// // 5, 11;
/// // 6, 12]
/// ```
/// 
/// * src1: first input array to be considered for vertical concatenation.
/// * src2: second input array to be considered for vertical concatenation.
/// * dst: output array. It has the same number of cols and depth as the src1 and src2, and the sum of rows of the src1 and src2.
pub fn vconcat_2(src1: &core::Mat, src2: &core::Mat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_vconcat_Mat_Mat_Mat(src1.as_raw_Mat(), src2.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
}

/// Applies vertical concatenation to given matrices.
/// 
/// The function vertically concatenates two or more cv::Mat matrices (with the same number of cols).
/// ```ignore{.cpp}
/// cv::Mat matArray[] = { cv::Mat(1, 4, CV_8UC1, cv::Scalar(1)),
/// cv::Mat(1, 4, CV_8UC1, cv::Scalar(2)),
/// cv::Mat(1, 4, CV_8UC1, cv::Scalar(3)),};
/// 
/// cv::Mat out;
/// cv::vconcat( matArray, 3, out );
/// //out:
/// //[1,   1,   1,   1;
/// // 2,   2,   2,   2;
/// // 3,   3,   3,   3]
/// ```
/// 
/// ## Parameters
/// * src: input array or vector of matrices. all of the matrices must have the same number of cols and the same depth.
/// * nsrc: number of matrices in src.
/// * dst: output array. It has the same number of cols and depth as the src, and the sum of rows of the src.
/// ## See also
/// cv::hconcat(const Mat*, size_t, OutputArray),  cv::hconcat(InputArrayOfArrays, OutputArray) and  cv::hconcat(InputArray, InputArray, OutputArray)
/// 
/// ## Overloaded parameters
/// 
/// ```ignore{.cpp}
/// std::vector<cv::Mat> matrices = { cv::Mat(1, 4, CV_8UC1, cv::Scalar(1)),
/// cv::Mat(1, 4, CV_8UC1, cv::Scalar(2)),
/// cv::Mat(1, 4, CV_8UC1, cv::Scalar(3)),};
/// 
/// cv::Mat out;
/// cv::vconcat( matrices, out );
/// //out:
/// //[1,   1,   1,   1;
/// // 2,   2,   2,   2;
/// // 3,   3,   3,   3]
/// ```
/// 
/// * src: input array or vector of matrices. all of the matrices must have the same number of cols and the same depth
/// * dst: output array. It has the same number of cols and depth as the src, and the sum of rows of the src.
/// same depth.
pub fn vconcat(src: &types::VectorOfMat, dst: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_vconcat_VectorOfMat_Mat(src.as_raw_VectorOfMat(), dst.as_raw_Mat()) }.into_result()
}

// Generating impl for trait cv::Algorithm (trait)
/// This is a base class for all more or less complex algorithms in OpenCV
/// 
/// especially for classes of algorithms, for which there can be multiple implementations. The examples
/// are stereo correspondence (for which there are algorithms like block matching, semi-global block
/// matching, graph-cut etc.), background subtraction (which can be done using mixture-of-gaussians
/// models, codebook-based algorithm etc.), optical flow (block matching, Lucas-Kanade, Horn-Schunck
/// etc.).
/// 
/// Here is example of SimpleBlobDetector use in your application via Algorithm interface:
/// @snippet snippets/core_various.cpp Algorithm
pub trait Algorithm {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void;
    /// Clears the algorithm state
    fn clear(&mut self) -> Result<()> {
        unsafe { sys::cv_Algorithm_clear(self.as_raw_Algorithm()) }.into_result()
    }
    
    /// Returns true if the Algorithm is empty (e.g. in the very beginning or after unsuccessful read
    fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_Algorithm_empty_const(self.as_raw_Algorithm()) }.into_result()
    }
    
    /// Saves the algorithm to a file.
    /// In order to make this method work, the derived class must implement Algorithm::write(FileStorage& fs).
    fn save(&self, filename: &str) -> Result<()> {
        string_arg!(filename);
        unsafe { sys::cv_Algorithm_save_const_String(self.as_raw_Algorithm(), filename.as_ptr()) }.into_result()
    }
    
    /// Returns the algorithm string identifier.
    /// This string is used as top level xml/yml node tag when the object is saved to a file or string.
    fn get_default_name(&self) -> Result<String> {
        unsafe { sys::cv_Algorithm_getDefaultName_const(self.as_raw_Algorithm()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

impl<'a> Algorithm + 'a {

}

// boxed class cv::AutoLock
#[allow(dead_code)]
pub struct AutoLock {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::AutoLock {
    fn drop(&mut self) {
        unsafe { sys::cv_AutoLock_delete(self.ptr) };
    }
}
impl core::AutoLock {
    pub fn as_raw_AutoLock(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        AutoLock {
            ptr
        }
    }
}

// Generating impl for trait cv::BufferPoolController (trait)
pub trait BufferPoolController {
    #[doc(hidden)] fn as_raw_BufferPoolController(&self) -> *mut c_void;
    fn get_reserved_size(&self) -> Result<size_t> {
        unsafe { sys::cv_BufferPoolController_getReservedSize_const(self.as_raw_BufferPoolController()) }.into_result()
    }
    
    fn get_max_reserved_size(&self) -> Result<size_t> {
        unsafe { sys::cv_BufferPoolController_getMaxReservedSize_const(self.as_raw_BufferPoolController()) }.into_result()
    }
    
    fn set_max_reserved_size(&mut self, size: size_t) -> Result<()> {
        unsafe { sys::cv_BufferPoolController_setMaxReservedSize_size_t(self.as_raw_BufferPoolController(), size) }.into_result()
    }
    
    fn free_all_reserved_buffers(&mut self) -> Result<()> {
        unsafe { sys::cv_BufferPoolController_freeAllReservedBuffers(self.as_raw_BufferPoolController()) }.into_result()
    }
    
}

impl<'a> BufferPoolController + 'a {

}

// boxed class cv::CommandLineParser
/// Designed for command line parsing
/// 
/// The sample below demonstrates how to use CommandLineParser:
/// ```ignore
/// CommandLineParser parser(argc, argv, keys);
/// parser.about("Application name v1.0.0");
/// 
/// if (parser.has("help"))
/// {
/// parser.printMessage();
/// return 0;
/// }
/// 
/// int N = parser.get<int>("N");
/// double fps = parser.get<double>("fps");
/// String path = parser.get<String>("path");
/// 
/// use_time_stamp = parser.has("timestamp");
/// 
/// String img1 = parser.get<String>(0);
/// String img2 = parser.get<String>(1);
/// 
/// int repeat = parser.get<int>(2);
/// 
/// if (!parser.check())
/// {
/// parser.printErrors();
/// return 0;
/// }
/// ```
/// 
/// 
/// ### Keys syntax
/// 
/// The keys parameter is a string containing several blocks, each one is enclosed in curly braces and
/// describes one argument. Each argument contains three parts separated by the `|` symbol:
/// 
/// -# argument names is a space-separated list of option synonyms (to mark argument as positional, prefix it with the `@` symbol)
/// -# default value will be used if the argument was not provided (can be empty)
/// -# help message (can be empty)
/// 
/// For example:
/// 
/// ```ignore{.cpp}
/// const String keys =
/// "{help h usage ? |      | print this message   }"
/// "{@image1        |      | image1 for compare   }"
/// "{@image2        |<none>| image2 for compare   }"
/// "{@repeat        |1     | number               }"
/// "{path           |.     | path to file         }"
/// "{fps            | -1.0 | fps for output video }"
/// "{N count        |100   | count of objects     }"
/// "{ts timestamp   |      | use time stamp       }"
/// ;
/// }
/// ```
/// 
/// 
/// Note that there are no default values for `help` and `timestamp` so we can check their presence using the `has()` method.
/// Arguments with default values are considered to be always present. Use the `get()` method in these cases to check their
/// actual value instead.
/// 
/// String keys like `get<String>("@image1")` return the empty string `""` by default - even with an empty default value.
/// Use the special `<none>` default value to enforce that the returned string must not be empty. (like in `get<String>("@image2")`)
/// 
/// ### Usage
/// 
/// For the described keys:
/// 
/// ```ignore{.sh}
/// # Good call (3 positional parameters: image1, image2 and repeat; N is 200, ts is true)
/// $ ./app -N=200 1.png 2.jpg 19 -ts
/// 
/// # Bad call
/// $ ./app -fps=aaa
/// ERRORS:
/// Parameter 'fps': can not convert: [aaa] to [double]
/// ```
#[allow(dead_code)]
pub struct CommandLineParser {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::CommandLineParser {
    fn drop(&mut self) {
        unsafe { sys::cv_CommandLineParser_delete(self.ptr) };
    }
}
impl core::CommandLineParser {
    pub fn as_raw_CommandLineParser(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        CommandLineParser {
            ptr
        }
    }
}

impl CommandLineParser {

    /// Copy constructor
    pub fn new(parser: &core::CommandLineParser) -> Result<core::CommandLineParser> {
        unsafe { sys::cv_CommandLineParser_CommandLineParser_CommandLineParser(parser.as_raw_CommandLineParser()) }.into_result().map(|x| core::CommandLineParser { ptr: x })
    }
    
    /// Returns application path
    /// 
    /// This method returns the path to the executable from the command line (`argv[0]`).
    /// 
    /// For example, if the application has been started with such a command:
    /// ```ignore{.sh}
    /// $ ./bin/my-executable
    /// ```
    /// 
    /// this method will return `./bin`.
    pub fn get_path_to_application(&self) -> Result<String> {
        unsafe { sys::cv_CommandLineParser_getPathToApplication_const(self.as_raw_CommandLineParser()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    /// Check if field was provided in the command line
    /// 
    /// ## Parameters
    /// * name: argument name to check
    pub fn has(&self, name: &str) -> Result<bool> {
        string_arg!(name);
        unsafe { sys::cv_CommandLineParser_has_const_String(self.as_raw_CommandLineParser(), name.as_ptr()) }.into_result()
    }
    
    /// Check for parsing errors
    /// 
    /// Returns false if error occurred while accessing the parameters (bad conversion, missing arguments,
    /// etc.). Call @ref printErrors to print error messages list.
    pub fn check(&self) -> Result<bool> {
        unsafe { sys::cv_CommandLineParser_check_const(self.as_raw_CommandLineParser()) }.into_result()
    }
    
    /// Set the about message
    /// 
    /// The about message will be shown when @ref printMessage is called, right before arguments table.
    pub fn about(&mut self, message: &str) -> Result<()> {
        string_arg!(message);
        unsafe { sys::cv_CommandLineParser_about_String(self.as_raw_CommandLineParser(), message.as_ptr()) }.into_result()
    }
    
    /// Print help message
    /// 
    /// This method will print standard help message containing the about message and arguments description.
    /// 
    /// ## See also
    /// about
    pub fn print_message(&self) -> Result<()> {
        unsafe { sys::cv_CommandLineParser_printMessage_const(self.as_raw_CommandLineParser()) }.into_result()
    }
    
    /// Print list of errors occurred
    /// 
    /// ## See also
    /// check
    pub fn print_errors(&self) -> Result<()> {
        unsafe { sys::cv_CommandLineParser_printErrors_const(self.as_raw_CommandLineParser()) }.into_result()
    }
    
}

// boxed class cv::ConjGradSolver
/// This class is used to perform the non-linear non-constrained minimization of a function
/// with known gradient,
/// 
/// defined on an *n*-dimensional Euclidean space, using the **Nonlinear Conjugate Gradient method**.
/// The implementation was done based on the beautifully clear explanatory article [An Introduction to
/// the Conjugate Gradient Method Without the Agonizing
/// Pain](http://www.cs.cmu.edu/~quake-papers/painless-conjugate-gradient.pdf) by Jonathan Richard
/// Shewchuk. The method can be seen as an adaptation of a standard Conjugate Gradient method (see, for
/// example <http://en.wikipedia.org/wiki/Conjugate_gradient_method>) for numerically solving the
/// systems of linear equations.
/// 
/// It should be noted, that this method, although deterministic, is rather a heuristic method and
/// therefore may converge to a local minima, not necessary a global one. What is even more disastrous,
/// most of its behaviour is ruled by gradient, therefore it essentially cannot distinguish between
/// local minima and maxima. Therefore, if it starts sufficiently near to the local maximum, it may
/// converge to it. Another obvious restriction is that it should be possible to compute the gradient of
/// a function at any point, thus it is preferable to have analytic expression for gradient and
/// computational burden should be born by the user.
/// 
/// The latter responsibility is accompilished via the getGradient method of a
/// MinProblemSolver::Function interface (which represents function being optimized). This method takes
/// point a point in *n*-dimensional space (first argument represents the array of coordinates of that
/// point) and comput its gradient (it should be stored in the second argument as an array).
/// 
/// 
/// Note: class ConjGradSolver thus does not add any new methods to the basic MinProblemSolver interface.
/// 
/// 
/// Note: term criteria should meet following condition:
/// ```ignore
/// termcrit.type == (TermCriteria::MAX_ITER + TermCriteria::EPS) && termcrit.epsilon > 0 && termcrit.maxCount > 0
/// // or
/// termcrit.type == TermCriteria::MAX_ITER) && termcrit.maxCount > 0
/// ```
#[allow(dead_code)]
pub struct ConjGradSolver {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::ConjGradSolver {
    fn drop(&mut self) {
        unsafe { sys::cv_ConjGradSolver_delete(self.ptr) };
    }
}
impl core::ConjGradSolver {
    pub fn as_raw_ConjGradSolver(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        ConjGradSolver {
            ptr
        }
    }
}

impl core::Algorithm for ConjGradSolver {
    fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl core::MinProblemSolver for ConjGradSolver {
    fn as_raw_MinProblemSolver(&self) -> *mut c_void { self.ptr }
}

impl ConjGradSolver {

    /// This function returns the reference to the ready-to-use ConjGradSolver object.
    /// 
    /// All the parameters are optional, so this procedure can be called even without parameters at
    /// all. In this case, the default values will be used. As default value for terminal criteria are
    /// the only sensible ones, MinProblemSolver::setFunction() should be called upon the obtained
    /// object, if the function was not given to create(). Otherwise, the two ways (submit it to
    /// create() or miss it out and call the MinProblemSolver::setFunction()) are absolutely equivalent
    /// (and will drop the same errors in the same way, should invalid input be detected).
    /// ## Parameters
    /// * f: Pointer to the function that will be minimized, similarly to the one you submit via
    /// MinProblemSolver::setFunction.
    /// * termcrit: Terminal criteria to the algorithm, similarly to the one you submit via
    /// MinProblemSolver::setTermCriteria.
    ///
    /// ## C++ default parameters
    /// * f: Ptr<ConjGradSolver::Function>()
    /// * termcrit: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5000,0.000001)
    pub fn create(f: &types::PtrOfFunction, termcrit: &core::TermCriteria) -> Result<types::PtrOfConjGradSolver> {
        unsafe { sys::cv_ConjGradSolver_create_PtrOfFunction_TermCriteria(f.as_raw_PtrOfFunction(), termcrit.as_raw_TermCriteria()) }.into_result().map(|x| types::PtrOfConjGradSolver { ptr: x })
    }
    
}

impl DMatch {

    pub fn default() -> Result<core::DMatch> {
        unsafe { sys::cv_DMatch_DMatch() }.into_result()
    }
    
    pub fn new(_query_idx: i32, _train_idx: i32, _distance: f32) -> Result<core::DMatch> {
        unsafe { sys::cv_DMatch_DMatch_int_int_float(_query_idx, _train_idx, _distance) }.into_result()
    }
    
    pub fn new_index(_query_idx: i32, _train_idx: i32, _img_idx: i32, _distance: f32) -> Result<core::DMatch> {
        unsafe { sys::cv_DMatch_DMatch_int_int_int_float(_query_idx, _train_idx, _img_idx, _distance) }.into_result()
    }
    
}

// Generating impl for trait cv::DownhillSolver (trait)
/// This class is used to perform the non-linear non-constrained minimization of a function,
/// 
/// defined on an `n`-dimensional Euclidean space, using the **Nelder-Mead method**, also known as
/// **downhill simplex method**. The basic idea about the method can be obtained from
/// <http://en.wikipedia.org/wiki/Nelder-Mead_method>.
/// 
/// It should be noted, that this method, although deterministic, is rather a heuristic and therefore
/// may converge to a local minima, not necessary a global one. It is iterative optimization technique,
/// which at each step uses an information about the values of a function evaluated only at `n+1`
/// points, arranged as a *simplex* in `n`-dimensional space (hence the second name of the method). At
/// each step new point is chosen to evaluate function at, obtained value is compared with previous
/// ones and based on this information simplex changes it's shape , slowly moving to the local minimum.
/// Thus this method is using *only* function values to make decision, on contrary to, say, Nonlinear
/// Conjugate Gradient method (which is also implemented in optim).
/// 
/// Algorithm stops when the number of function evaluations done exceeds termcrit.maxCount, when the
/// function values at the vertices of simplex are within termcrit.epsilon range or simplex becomes so
/// small that it can enclosed in a box with termcrit.epsilon sides, whatever comes first, for some
/// defined by user positive integer termcrit.maxCount and positive non-integer termcrit.epsilon.
/// 
/// 
/// Note: DownhillSolver is a derivative of the abstract interface
/// cv::MinProblemSolver, which in turn is derived from the Algorithm interface and is used to
/// encapsulate the functionality, common to all non-linear optimization algorithms in the optim
/// module.
/// 
/// 
/// Note: term criteria should meet following condition:
/// ```ignore
/// termcrit.type == (TermCriteria::MAX_ITER + TermCriteria::EPS) && termcrit.epsilon > 0 && termcrit.maxCount > 0
/// ```
pub trait DownhillSolver : core::MinProblemSolver {
    #[doc(hidden)] fn as_raw_DownhillSolver(&self) -> *mut c_void;
    /// Returns the initial step that will be used in downhill simplex algorithm.
    /// 
    /// ## Parameters
    /// * step: Initial step that will be used in algorithm. Note, that although corresponding setter
    /// accepts column-vectors as well as row-vectors, this method will return a row-vector.
    /// @see DownhillSolver::setInitStep
    fn get_init_step(&self, step: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_DownhillSolver_getInitStep_const_Mat(self.as_raw_DownhillSolver(), step.as_raw_Mat()) }.into_result()
    }
    
    /// Sets the initial step that will be used in downhill simplex algorithm.
    /// 
    /// Step, together with initial point (givin in DownhillSolver::minimize) are two `n`-dimensional
    /// vectors that are used to determine the shape of initial simplex. Roughly said, initial point
    /// determines the position of a simplex (it will become simplex's centroid), while step determines the
    /// spread (size in each dimension) of a simplex. To be more precise, if <span lang='latex'>s,x_0\in\mathbb{R}^n</span> are
    /// the initial step and initial point respectively, the vertices of a simplex will be:
    /// <span lang='latex'>v_0:=x_0-\frac{1}{2} s</span> and <span lang='latex'>v_i:=x_0+s_i</span> for <span lang='latex'>i=1,2,\dots,n</span> where <span lang='latex'>s_i</span> denotes
    /// projections of the initial step of *n*-th coordinate (the result of projection is treated to be
    /// vector given by <span lang='latex'>s_i:=e_i\cdot\left<e_i\cdot s\right></span>, where <span lang='latex'>e_i</span> form canonical basis)
    /// 
    /// ## Parameters
    /// * step: Initial step that will be used in algorithm. Roughly said, it determines the spread
    /// (size in each dimension) of an initial simplex.
    fn set_init_step(&mut self, step: &core::Mat) -> Result<()> {
        unsafe { sys::cv_DownhillSolver_setInitStep_Mat(self.as_raw_DownhillSolver(), step.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> DownhillSolver + 'a {

    /// This function returns the reference to the ready-to-use DownhillSolver object.
    /// 
    /// All the parameters are optional, so this procedure can be called even without parameters at
    /// all. In this case, the default values will be used. As default value for terminal criteria are
    /// the only sensible ones, MinProblemSolver::setFunction() and DownhillSolver::setInitStep()
    /// should be called upon the obtained object, if the respective parameters were not given to
    /// create(). Otherwise, the two ways (give parameters to createDownhillSolver() or miss them out
    /// and call the MinProblemSolver::setFunction() and DownhillSolver::setInitStep()) are absolutely
    /// equivalent (and will drop the same errors in the same way, should invalid input be detected).
    /// ## Parameters
    /// * f: Pointer to the function that will be minimized, similarly to the one you submit via
    /// MinProblemSolver::setFunction.
    /// * initStep: Initial step, that will be used to construct the initial simplex, similarly to the one
    /// you submit via MinProblemSolver::setInitStep.
    /// * termcrit: Terminal criteria to the algorithm, similarly to the one you submit via
    /// MinProblemSolver::setTermCriteria.
    ///
    /// ## C++ default parameters
    /// * f: Ptr<MinProblemSolver::Function>()
    /// * init_step: Mat_<double>(1,1,0.0)
    /// * termcrit: TermCriteria(TermCriteria::MAX_ITER+TermCriteria::EPS,5000,0.000001)
    pub fn create(f: &types::PtrOfFunction, init_step: &core::Mat, termcrit: &core::TermCriteria) -> Result<types::PtrOfDownhillSolver> {
        unsafe { sys::cv_DownhillSolver_create_PtrOfFunction_Mat_TermCriteria(f.as_raw_PtrOfFunction(), init_step.as_raw_Mat(), termcrit.as_raw_TermCriteria()) }.into_result().map(|x| types::PtrOfDownhillSolver { ptr: x })
    }
    
}

// Generating impl for trait cv::Formatted (trait)
/// @todo document
pub trait Formatted {
    #[doc(hidden)] fn as_raw_Formatted(&self) -> *mut c_void;
    fn next(&mut self) -> Result<String> {
        unsafe { sys::cv_Formatted_next(self.as_raw_Formatted()) }.into_result().map(crate::templ::receive_string)
    }
    
    fn reset(&mut self) -> Result<()> {
        unsafe { sys::cv_Formatted_reset(self.as_raw_Formatted()) }.into_result()
    }
    
}

impl<'a> Formatted + 'a {

}

// Generating impl for trait cv::Formatter (trait)
/// @todo document
pub trait Formatter {
    #[doc(hidden)] fn as_raw_Formatter(&self) -> *mut c_void;
    fn format(&self, mtx: &core::Mat) -> Result<types::PtrOfFormatted> {
        unsafe { sys::cv_Formatter_format_const_Mat(self.as_raw_Formatter(), mtx.as_raw_Mat()) }.into_result().map(|x| types::PtrOfFormatted { ptr: x })
    }
    
    ///
    /// ## C++ default parameters
    /// * p: 8
    fn set32f_precision(&mut self, p: i32) -> Result<()> {
        unsafe { sys::cv_Formatter_set32fPrecision_int(self.as_raw_Formatter(), p) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * p: 16
    fn set64f_precision(&mut self, p: i32) -> Result<()> {
        unsafe { sys::cv_Formatter_set64fPrecision_int(self.as_raw_Formatter(), p) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * ml: true
    fn set_multiline(&mut self, ml: bool) -> Result<()> {
        unsafe { sys::cv_Formatter_setMultiline_bool(self.as_raw_Formatter(), ml) }.into_result()
    }
    
}

impl<'a> Formatter + 'a {

    ///
    /// ## C++ default parameters
    /// * fmt: FMT_DEFAULT
    pub fn get(fmt: i32) -> Result<types::PtrOfFormatter> {
        unsafe { sys::cv_Formatter_get_int(fmt) }.into_result().map(|x| types::PtrOfFormatter { ptr: x })
    }
    
}

// boxed class cv::Hamming
/// replaced with CV_Assert(expr) in Debug configuration
#[allow(dead_code)]
pub struct Hamming {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::Hamming {
    fn drop(&mut self) {
        unsafe { sys::cv_Hamming_delete(self.ptr) };
    }
}
impl core::Hamming {
    pub fn as_raw_Hamming(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Hamming {
            ptr
        }
    }
}

impl KeyPoint {

    /// the default constructor
    pub fn default() -> Result<core::KeyPoint> {
        unsafe { sys::cv_KeyPoint_KeyPoint() }.into_result()
    }
    
    /// ## Parameters
    /// * _pt: x & y coordinates of the keypoint
    /// * _size: keypoint diameter
    /// * _angle: keypoint orientation
    /// * _response: keypoint detector response on the keypoint (that is, strength of the keypoint)
    /// * _octave: pyramid octave in which the keypoint has been detected
    /// * _class_id: object id
    ///
    /// ## C++ default parameters
    /// * _angle: -1
    /// * _response: 0
    /// * _octave: 0
    /// * _class_id: -1
    pub fn new_point(_pt: core::Point2f, _size: f32, _angle: f32, _response: f32, _octave: i32, _class_id: i32) -> Result<core::KeyPoint> {
        unsafe { sys::cv_KeyPoint_KeyPoint_Point2f_float_float_float_int_int(_pt, _size, _angle, _response, _octave, _class_id) }.into_result()
    }
    
    /// ## Parameters
    /// * x: x-coordinate of the keypoint
    /// * y: y-coordinate of the keypoint
    /// * _size: keypoint diameter
    /// * _angle: keypoint orientation
    /// * _response: keypoint detector response on the keypoint (that is, strength of the keypoint)
    /// * _octave: pyramid octave in which the keypoint has been detected
    /// * _class_id: object id
    ///
    /// ## C++ default parameters
    /// * _angle: -1
    /// * _response: 0
    /// * _octave: 0
    /// * _class_id: -1
    pub fn new_coords(x: f32, y: f32, _size: f32, _angle: f32, _response: f32, _octave: i32, _class_id: i32) -> Result<core::KeyPoint> {
        unsafe { sys::cv_KeyPoint_KeyPoint_float_float_float_float_float_int_int(x, y, _size, _angle, _response, _octave, _class_id) }.into_result()
    }
    
    pub fn hash(self) -> Result<size_t> {
        unsafe { sys::cv_KeyPoint_hash_const(self) }.into_result()
    }
    
    /// This method converts vector of keypoints to vector of points or the reverse, where each keypoint is
    /// assigned the same size and the same orientation.
    /// 
    /// ## Parameters
    /// * keypoints: Keypoints obtained from any feature detection algorithm like SIFT/SURF/ORB
    /// * points2f: Array of (x,y) coordinates of each keypoint
    /// * keypointIndexes: Array of indexes of keypoints to be converted to points. (Acts like a mask to
    /// convert only specified keypoints)
    ///
    /// ## C++ default parameters
    /// * keypoint_indexes: std::vector<int>()
    pub fn convert_from(keypoints: &types::VectorOfKeyPoint, points2f: &mut types::VectorOfPoint2f, keypoint_indexes: &types::VectorOfint) -> Result<()> {
        unsafe { sys::cv_KeyPoint_convert_VectorOfKeyPoint_VectorOfPoint2f_VectorOfint(keypoints.as_raw_VectorOfKeyPoint(), points2f.as_raw_VectorOfPoint2f(), keypoint_indexes.as_raw_VectorOfint()) }.into_result()
    }
    
    /// ## Parameters
    /// * points2f: Array of (x,y) coordinates of each keypoint
    /// * keypoints: Keypoints obtained from any feature detection algorithm like SIFT/SURF/ORB
    /// * size: keypoint diameter
    /// * response: keypoint detector response on the keypoint (that is, strength of the keypoint)
    /// * octave: pyramid octave in which the keypoint has been detected
    /// * class_id: object id
    ///
    /// ## C++ default parameters
    /// * size: 1
    /// * response: 1
    /// * octave: 0
    /// * class_id: -1
    pub fn convert_to(points2f: &types::VectorOfPoint2f, keypoints: &mut types::VectorOfKeyPoint, size: f32, response: f32, octave: i32, class_id: i32) -> Result<()> {
        unsafe { sys::cv_KeyPoint_convert_VectorOfPoint2f_VectorOfKeyPoint_float_float_int_int(points2f.as_raw_VectorOfPoint2f(), keypoints.as_raw_VectorOfKeyPoint(), size, response, octave, class_id) }.into_result()
    }
    
    /// This method computes overlap for pair of keypoints. Overlap is the ratio between area of keypoint
    /// regions' intersection and area of keypoint regions' union (considering keypoint region as circle).
    /// If they don't overlap, we get zero. If they coincide at same location with same size, we get 1.
    /// ## Parameters
    /// * kp1: First keypoint
    /// * kp2: Second keypoint
    pub fn overlap(kp1: core::KeyPoint, kp2: core::KeyPoint) -> Result<f32> {
        unsafe { sys::cv_KeyPoint_overlap_KeyPoint_KeyPoint(kp1, kp2) }.into_result()
    }
    
}

// boxed class cv::LDA
/// Linear Discriminant Analysis
/// @todo document this class
#[allow(dead_code)]
pub struct LDA {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::LDA {
    fn drop(&mut self) {
        unsafe { sys::cv_LDA_delete(self.ptr) };
    }
}
impl core::LDA {
    pub fn as_raw_LDA(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        LDA {
            ptr
        }
    }
}

impl LDA {

    /// constructor
    /// Initializes a LDA with num_components (default 0).
    ///
    /// ## C++ default parameters
    /// * num_components: 0
    pub fn new(num_components: i32) -> Result<core::LDA> {
        unsafe { sys::cv_LDA_LDA_int(num_components) }.into_result().map(|x| core::LDA { ptr: x })
    }
    
    /// Initializes and performs a Discriminant Analysis with Fisher's
    /// Optimization Criterion on given data in src and corresponding labels
    /// in labels. If 0 (or less) number of components are given, they are
    /// automatically determined for given data in computation.
    ///
    /// ## C++ default parameters
    /// * num_components: 0
    pub fn new_with_data(src: &types::VectorOfMat, labels: &core::Mat, num_components: i32) -> Result<core::LDA> {
        unsafe { sys::cv_LDA_LDA_VectorOfMat_Mat_int(src.as_raw_VectorOfMat(), labels.as_raw_Mat(), num_components) }.into_result().map(|x| core::LDA { ptr: x })
    }
    
    /// Serializes this object to a given filename.
    pub fn save(&self, filename: &str) -> Result<()> {
        string_arg!(filename);
        unsafe { sys::cv_LDA_save_const_String(self.as_raw_LDA(), filename.as_ptr()) }.into_result()
    }
    
    /// Deserializes this object from a given filename.
    pub fn load(&mut self, filename: &str) -> Result<()> {
        string_arg!(filename);
        unsafe { sys::cv_LDA_load_String(self.as_raw_LDA(), filename.as_ptr()) }.into_result()
    }
    
    /// Compute the discriminants for data in src (row aligned) and labels.
    pub fn compute(&mut self, src: &types::VectorOfMat, labels: &core::Mat) -> Result<()> {
        unsafe { sys::cv_LDA_compute_VectorOfMat_Mat(self.as_raw_LDA(), src.as_raw_VectorOfMat(), labels.as_raw_Mat()) }.into_result()
    }
    
    /// Projects samples into the LDA subspace.
    /// src may be one or more row aligned samples.
    pub fn project(&mut self, src: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_LDA_project_Mat(self.as_raw_LDA(), src.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Reconstructs projections from the LDA subspace.
    /// src may be one or more row aligned projections.
    pub fn reconstruct(&mut self, src: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_LDA_reconstruct_Mat(self.as_raw_LDA(), src.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Returns the eigenvectors of this LDA.
    pub fn eigenvectors(&self) -> Result<core::Mat> {
        unsafe { sys::cv_LDA_eigenvectors_const(self.as_raw_LDA()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Returns the eigenvalues of this LDA.
    pub fn eigenvalues(&self) -> Result<core::Mat> {
        unsafe { sys::cv_LDA_eigenvalues_const(self.as_raw_LDA()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    pub fn subspace_project(w: &core::Mat, mean: &core::Mat, src: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_LDA_subspaceProject_Mat_Mat_Mat(w.as_raw_Mat(), mean.as_raw_Mat(), src.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    pub fn subspace_reconstruct(w: &core::Mat, mean: &core::Mat, src: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_LDA_subspaceReconstruct_Mat_Mat_Mat(w.as_raw_Mat(), mean.as_raw_Mat(), src.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
}

// boxed class cv::Mat
/// n-dimensional dense array class \anchor CVMat_Details
/// 
/// The class Mat represents an n-dimensional dense numerical single-channel or multi-channel array. It
/// can be used to store real or complex-valued vectors and matrices, grayscale or color images, voxel
/// volumes, vector fields, point clouds, tensors, histograms (though, very high-dimensional histograms
/// may be better stored in a SparseMat ). The data layout of the array `M` is defined by the array
/// `M.step[]`, so that the address of element <span lang='latex'>(i_0,...,i_{M.dims-1})</span>, where <span lang='latex'>0\leq i_k<M.size[k]</span>, is
/// computed as:
/// <div lang='latex'>addr(M_{i_0,...,i_{M.dims-1}}) = M.data + M.step[0]*i_0 + M.step[1]*i_1 + ... + M.step[M.dims-1]*i_{M.dims-1}</div>
/// In case of a 2-dimensional array, the above formula is reduced to:
/// <div lang='latex'>addr(M_{i,j}) = M.data + M.step[0]*i + M.step[1]*j</div>
/// Note that `M.step[i] >= M.step[i+1]` (in fact, `M.step[i] >= M.step[i+1]*M.size[i+1]` ). This means
/// that 2-dimensional matrices are stored row-by-row, 3-dimensional matrices are stored plane-by-plane,
/// and so on. M.step[M.dims-1] is minimal and always equal to the element size M.elemSize() .
/// 
/// So, the data layout in Mat is fully compatible with CvMat, IplImage, and CvMatND types from OpenCV
/// 1.x. It is also compatible with the majority of dense array types from the standard toolkits and
/// SDKs, such as Numpy (ndarray), Win32 (independent device bitmaps), and others, that is, with any
/// array that uses *steps* (or *strides*) to compute the position of a pixel. Due to this
/// compatibility, it is possible to make a Mat header for user-allocated data and process it in-place
/// using OpenCV functions.
/// 
/// There are many different ways to create a Mat object. The most popular options are listed below:
/// 
/// - Use the create(nrows, ncols, type) method or the similar Mat(nrows, ncols, type[, fillValue])
/// constructor. A new array of the specified size and type is allocated. type has the same meaning as
/// in the cvCreateMat method. For example, CV_8UC1 means a 8-bit single-channel array, CV_32FC2
/// means a 2-channel (complex) floating-point array, and so on.
/// ```ignore
/// // make a 7x7 complex matrix filled with 1+3j.
/// Mat M(7,7,CV_32FC2,Scalar(1,3));
/// // and now turn M to a 100x60 15-channel 8-bit matrix.
/// // The old content will be deallocated
/// M.create(100,60,CV_8UC(15));
/// ```
/// 
/// As noted in the introduction to this chapter, create() allocates only a new array when the shape
/// or type of the current array are different from the specified ones.
/// 
/// - Create a multi-dimensional array:
/// ```ignore
/// // create a 100x100x100 8-bit array
/// int sz[] = {100, 100, 100};
/// Mat bigCube(3, sz, CV_8U, Scalar::all(0));
/// ```
/// 
/// It passes the number of dimensions =1 to the Mat constructor but the created array will be
/// 2-dimensional with the number of columns set to 1. So, Mat::dims is always \>= 2 (can also be 0
/// when the array is empty).
/// 
/// - Use a copy constructor or assignment operator where there can be an array or expression on the
/// right side (see below). As noted in the introduction, the array assignment is an O(1) operation
/// because it only copies the header and increases the reference counter. The Mat::clone() method can
/// be used to get a full (deep) copy of the array when you need it.
/// 
/// - Construct a header for a part of another array. It can be a single row, single column, several
/// rows, several columns, rectangular region in the array (called a *minor* in algebra) or a
/// diagonal. Such operations are also O(1) because the new header references the same data. You can
/// actually modify a part of the array using this feature, for example:
/// ```ignore
/// // add the 5-th row, multiplied by 3 to the 3rd row
/// M.row(3) = M.row(3) + M.row(5)*3;
/// // now copy the 7-th column to the 1-st column
/// // M.col(1) = M.col(7); // this will not work
/// Mat M1 = M.col(1);
/// M.col(7).copyTo(M1);
/// // create a new 320x240 image
/// Mat img(Size(320,240),CV_8UC3);
/// // select a ROI
/// Mat roi(img, Rect(10,10,100,100));
/// // fill the ROI with (0,255,0) (which is green in RGB space);
/// // the original 320x240 image will be modified
/// roi = Scalar(0,255,0);
/// ```
/// 
/// Due to the additional datastart and dataend members, it is possible to compute a relative
/// sub-array position in the main *container* array using locateROI():
/// ```ignore
/// Mat A = Mat::eye(10, 10, CV_32S);
/// // extracts A columns, 1 (inclusive) to 3 (exclusive).
/// Mat B = A(Range::all(), Range(1, 3));
/// // extracts B rows, 5 (inclusive) to 9 (exclusive).
/// // that is, C \~ A(Range(5, 9), Range(1, 3))
/// Mat C = B(Range(5, 9), Range::all());
/// Size size; Point ofs;
/// C.locateROI(size, ofs);
/// // size will be (width=10,height=10) and the ofs will be (x=1, y=5)
/// ```
/// 
/// As in case of whole matrices, if you need a deep copy, use the `clone()` method of the extracted
/// sub-matrices.
/// 
/// - Make a header for user-allocated data. It can be useful to do the following:
/// -# Process "foreign" data using OpenCV (for example, when you implement a DirectShow\* filter or
/// a processing module for gstreamer, and so on). For example:
/// ```ignore
/// void process_video_frame(const unsigned char* pixels,
/// int width, int height, int step)
/// {
/// Mat img(height, width, CV_8UC3, pixels, step);
/// GaussianBlur(img, img, Size(7,7), 1.5, 1.5);
/// }
/// ```
/// 
/// -# Quickly initialize small matrices and/or get a super-fast element access.
/// ```ignore
/// double m[3][3] = {{a, b, c}, {d, e, f}, {g, h, i}};
/// Mat M = Mat(3, 3, CV_64F, m).inv();
/// ```
/// 
/// .
/// Partial yet very common cases of this *user-allocated data* case are conversions from CvMat and
/// IplImage to Mat. For this purpose, there is function cv::cvarrToMat taking pointers to CvMat or
/// IplImage and the optional flag indicating whether to copy the data or not.
/// @snippet samples/cpp/image.cpp iplimage
/// 
/// - Use MATLAB-style array initializers, zeros(), ones(), eye(), for example:
/// ```ignore
/// // create a double-precision identity matrix and add it to M.
/// M += Mat::eye(M.rows, M.cols, CV_64F);
/// ```
/// 
/// 
/// - Use a comma-separated initializer:
/// ```ignore
/// // create a 3x3 double-precision identity matrix
/// Mat M = (Mat_<double>(3,3) << 1, 0, 0, 0, 1, 0, 0, 0, 1);
/// ```
/// 
/// With this approach, you first call a constructor of the Mat class with the proper parameters, and
/// then you just put `<< operator` followed by comma-separated values that can be constants,
/// variables, expressions, and so on. Also, note the extra parentheses required to avoid compilation
/// errors.
/// 
/// Once the array is created, it is automatically managed via a reference-counting mechanism. If the
/// array header is built on top of user-allocated data, you should handle the data by yourself. The
/// array data is deallocated when no one points to it. If you want to release the data pointed by a
/// array header before the array destructor is called, use Mat::release().
/// 
/// The next important thing to learn about the array class is element access. This manual already
/// described how to compute an address of each array element. Normally, you are not required to use the
/// formula directly in the code. If you know the array element type (which can be retrieved using the
/// method Mat::type() ), you can access the element <span lang='latex'>M_{ij}</span> of a 2-dimensional array as:
/// ```ignore
/// M.at<double>(i,j) += 1.f;
/// ```
/// 
/// assuming that `M` is a double-precision floating-point array. There are several variants of the method
/// at for a different number of dimensions.
/// 
/// If you need to process a whole row of a 2D array, the most efficient way is to get the pointer to
/// the row first, and then just use the plain C operator [] :
/// ```ignore
/// // compute sum of positive matrix elements
/// // (assuming that M is a double-precision matrix)
/// double sum=0;
/// for(int i = 0; i < M.rows; i++)
/// {
/// const double* Mi = M.ptr<double>(i);
/// for(int j = 0; j < M.cols; j++)
/// sum += std::max(Mi[j], 0.);
/// }
/// ```
/// 
/// Some operations, like the one above, do not actually depend on the array shape. They just process
/// elements of an array one by one (or elements from multiple arrays that have the same coordinates,
/// for example, array addition). Such operations are called *element-wise*. It makes sense to check
/// whether all the input/output arrays are continuous, namely, have no gaps at the end of each row. If
/// yes, process them as a long single row:
/// ```ignore
/// // compute the sum of positive matrix elements, optimized variant
/// double sum=0;
/// int cols = M.cols, rows = M.rows;
/// if(M.isContinuous())
/// {
/// cols *= rows;
/// rows = 1;
/// }
/// for(int i = 0; i < rows; i++)
/// {
/// const double* Mi = M.ptr<double>(i);
/// for(int j = 0; j < cols; j++)
/// sum += std::max(Mi[j], 0.);
/// }
/// ```
/// 
/// In case of the continuous matrix, the outer loop body is executed just once. So, the overhead is
/// smaller, which is especially noticeable in case of small matrices.
/// 
/// Finally, there are STL-style iterators that are smart enough to skip gaps between successive rows:
/// ```ignore
/// // compute sum of positive matrix elements, iterator-based variant
/// double sum=0;
/// MatConstIterator_<double> it = M.begin<double>(), it_end = M.end<double>();
/// for(; it != it_end; ++it)
/// sum += std::max(*it, 0.);
/// ```
/// 
/// The matrix iterators are random-access iterators, so they can be passed to any STL algorithm,
/// including std::sort().
/// 
/// 
/// Note: Matrix Expressions and arithmetic see MatExpr
#[allow(dead_code)]
pub struct Mat {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::Mat {
    fn drop(&mut self) {
        unsafe { sys::cv_Mat_delete(self.ptr) };
    }
}
impl core::Mat {
    pub fn as_raw_Mat(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Mat {
            ptr
        }
    }
}

impl Mat {

    /// These are various constructors that form a matrix. As noted in the AutomaticAllocation, often
    /// the default constructor is enough, and the proper matrix will be allocated by an OpenCV function.
    /// The constructed matrix can further be assigned to another matrix or matrix expression or can be
    /// allocated with Mat::create . In the former case, the old content is de-referenced.
    pub fn new() -> Result<core::Mat> {
        unsafe { sys::cv_Mat_Mat() }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * rows: Number of rows in a 2D array.
    /// * cols: Number of columns in a 2D array.
    /// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
    /// CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
    pub unsafe fn new_rows_cols(rows: i32, cols: i32, _type: i32) -> Result<core::Mat> {
        { sys::cv_Mat_Mat_int_int_int(rows, cols, _type) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * size: 2D array size: Size(cols, rows) . In the Size() constructor, the number of rows and the
    /// number of columns go in the reverse order.
    /// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
    /// CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
    pub unsafe fn new_size(size: core::Size, _type: i32) -> Result<core::Mat> {
        { sys::cv_Mat_Mat_Size_int(size, _type) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * rows: Number of rows in a 2D array.
    /// * cols: Number of columns in a 2D array.
    /// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
    /// CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
    /// * s: An optional value to initialize each matrix element with. To set all the matrix elements to
    /// the particular value after the construction, use the assignment operator
    /// Mat::operator=(const Scalar& value) .
    pub fn new_rows_cols_with_default(rows: i32, cols: i32, _type: i32, s: core::Scalar) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_Mat_int_int_int_Scalar(rows, cols, _type, s) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * size: 2D array size: Size(cols, rows) . In the Size() constructor, the number of rows and the
    /// number of columns go in the reverse order.
    /// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
    /// CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
    /// * s: An optional value to initialize each matrix element with. To set all the matrix elements to
    /// the particular value after the construction, use the assignment operator
    /// Mat::operator=(const Scalar& value) .
    pub fn new_size_with_default(size: core::Size, _type: i32, s: core::Scalar) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_Mat_Size_int_Scalar(size, _type, s) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * sizes: Array of integers specifying an n-dimensional array shape.
    /// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
    /// CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
    pub unsafe fn new_nd(sizes: &types::VectorOfint, _type: i32) -> Result<core::Mat> {
        { sys::cv_Mat_Mat_VectorOfint_int(sizes.as_raw_VectorOfint(), _type) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * sizes: Array of integers specifying an n-dimensional array shape.
    /// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
    /// CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
    /// * s: An optional value to initialize each matrix element with. To set all the matrix elements to
    /// the particular value after the construction, use the assignment operator
    /// Mat::operator=(const Scalar& value) .
    pub fn new_nd_with_default(sizes: &types::VectorOfint, _type: i32, s: core::Scalar) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_Mat_VectorOfint_int_Scalar(sizes.as_raw_VectorOfint(), _type, s) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * m: Array that (as a whole or partly) is assigned to the constructed matrix. No data is copied
    /// by these constructors. Instead, the header pointing to m data or its sub-array is constructed and
    /// associated with it. The reference counter, if any, is incremented. So, when you modify the matrix
    /// formed using such a constructor, you also modify the corresponding elements of m . If you want to
    /// have an independent copy of the sub-array, use Mat::clone() .
    pub fn copy(m: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_Mat_Mat(m.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * rows: Number of rows in a 2D array.
    /// * cols: Number of columns in a 2D array.
    /// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
    /// CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
    /// * data: Pointer to the user data. Matrix constructors that take data and step parameters do not
    /// allocate matrix data. Instead, they just initialize the matrix header that points to the specified
    /// data, which means that no data is copied. This operation is very efficient and can be used to
    /// process external data using OpenCV functions. The external data is not automatically deallocated, so
    /// you should take care of it.
    /// * step: Number of bytes each matrix row occupies. The value should include the padding bytes at
    /// the end of each row, if any. If the parameter is missing (set to AUTO_STEP ), no padding is assumed
    /// and the actual step is calculated as cols*elemSize(). See Mat::elemSize.
    ///
    /// ## C++ default parameters
    /// * step: AUTO_STEP
    pub fn new_rows_cols_with_data(rows: i32, cols: i32, _type: i32, data: &mut c_void, step: size_t) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_Mat_int_int_int_void_X_size_t(rows, cols, _type, data, step) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * size: 2D array size: Size(cols, rows) . In the Size() constructor, the number of rows and the
    /// number of columns go in the reverse order.
    /// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
    /// CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
    /// * data: Pointer to the user data. Matrix constructors that take data and step parameters do not
    /// allocate matrix data. Instead, they just initialize the matrix header that points to the specified
    /// data, which means that no data is copied. This operation is very efficient and can be used to
    /// process external data using OpenCV functions. The external data is not automatically deallocated, so
    /// you should take care of it.
    /// * step: Number of bytes each matrix row occupies. The value should include the padding bytes at
    /// the end of each row, if any. If the parameter is missing (set to AUTO_STEP ), no padding is assumed
    /// and the actual step is calculated as cols*elemSize(). See Mat::elemSize.
    ///
    /// ## C++ default parameters
    /// * step: AUTO_STEP
    pub fn new_size_with_data(size: core::Size, _type: i32, data: &mut c_void, step: size_t) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_Mat_Size_int_void_X_size_t(size, _type, data, step) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * sizes: Array of integers specifying an n-dimensional array shape.
    /// * type: Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or
    /// CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
    /// * data: Pointer to the user data. Matrix constructors that take data and step parameters do not
    /// allocate matrix data. Instead, they just initialize the matrix header that points to the specified
    /// data, which means that no data is copied. This operation is very efficient and can be used to
    /// process external data using OpenCV functions. The external data is not automatically deallocated, so
    /// you should take care of it.
    /// * steps: Array of ndims-1 steps in case of a multi-dimensional array (the last step is always
    /// set to the element size). If not specified, the matrix is assumed to be continuous.
    ///
    /// ## C++ default parameters
    /// * steps: 0
    pub fn new_nd_with_data(sizes: &types::VectorOfint, _type: i32, data: &mut c_void, steps: &size_t) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_Mat_VectorOfint_int_void_X_const_size_t_X(sizes.as_raw_VectorOfint(), _type, data, steps) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * m: Array that (as a whole or partly) is assigned to the constructed matrix. No data is copied
    /// by these constructors. Instead, the header pointing to m data or its sub-array is constructed and
    /// associated with it. The reference counter, if any, is incremented. So, when you modify the matrix
    /// formed using such a constructor, you also modify the corresponding elements of m . If you want to
    /// have an independent copy of the sub-array, use Mat::clone() .
    /// * rowRange: Range of the m rows to take. As usual, the range start is inclusive and the range
    /// end is exclusive. Use Range::all() to take all the rows.
    /// * colRange: Range of the m columns to take. Use Range::all() to take all the columns.
    ///
    /// ## C++ default parameters
    /// * col_range: Range::all()
    pub fn rowscols(m: &core::Mat, row_range: &core::Range, col_range: &core::Range) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_Mat_Mat_Range_Range(m.as_raw_Mat(), row_range.as_raw_Range(), col_range.as_raw_Range()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * m: Array that (as a whole or partly) is assigned to the constructed matrix. No data is copied
    /// by these constructors. Instead, the header pointing to m data or its sub-array is constructed and
    /// associated with it. The reference counter, if any, is incremented. So, when you modify the matrix
    /// formed using such a constructor, you also modify the corresponding elements of m . If you want to
    /// have an independent copy of the sub-array, use Mat::clone() .
    /// * roi: Region of interest.
    pub fn roi(m: &core::Mat, roi: core::Rect) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_Mat_Mat_Rect(m.as_raw_Mat(), roi) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * m: Array that (as a whole or partly) is assigned to the constructed matrix. No data is copied
    /// by these constructors. Instead, the header pointing to m data or its sub-array is constructed and
    /// associated with it. The reference counter, if any, is incremented. So, when you modify the matrix
    /// formed using such a constructor, you also modify the corresponding elements of m . If you want to
    /// have an independent copy of the sub-array, use Mat::clone() .
    /// * ranges: Array of selected ranges of m along each dimensionality.
    pub fn ranges(m: &core::Mat, ranges: &types::VectorOfRange) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_Mat_Mat_VectorOfRange(m.as_raw_Mat(), ranges.as_raw_VectorOfRange()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Creates a matrix header for the specified matrix row.
    /// 
    /// The method makes a new header for the specified matrix row and returns it. This is an O(1)
    /// operation, regardless of the matrix size. The underlying data of the new matrix is shared with the
    /// original matrix. Here is the example of one of the classical basic matrix processing operations,
    /// axpy, used by LU and many other algorithms:
    /// ```ignore
    /// inline void matrix_axpy(Mat& A, int i, int j, double alpha)
    /// {
    /// A.row(i) += A.row(j)*alpha;
    /// }
    /// ```
    /// 
    /// 
    /// Note: In the current implementation, the following code does not work as expected:
    /// ```ignore
    /// Mat A;
    /// ...
    /// A.row(i) = A.row(j); // will not work
    /// ```
    /// 
    /// This happens because A.row(i) forms a temporary header that is further assigned to another header.
    /// Remember that each of these operations is O(1), that is, no data is copied. Thus, the above
    /// assignment is not true if you may have expected the j-th row to be copied to the i-th row. To
    /// achieve that, you should either turn this simple assignment into an expression or use the
    /// Mat::copyTo method:
    /// ```ignore
    /// Mat A;
    /// ...
    /// // works, but looks a bit obscure.
    /// A.row(i) = A.row(j) + 0;
    /// // this is a bit longer, but the recommended method.
    /// A.row(j).copyTo(A.row(i));
    /// ```
    /// 
    /// ## Parameters
    /// * y: A 0-based row index.
    pub fn row(&self, y: i32) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_row_const_int(self.as_raw_Mat(), y) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Creates a matrix header for the specified matrix column.
    /// 
    /// The method makes a new header for the specified matrix column and returns it. This is an O(1)
    /// operation, regardless of the matrix size. The underlying data of the new matrix is shared with the
    /// original matrix. See also the Mat::row description.
    /// ## Parameters
    /// * x: A 0-based column index.
    pub fn col(&self, x: i32) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_col_const_int(self.as_raw_Mat(), x) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Creates a matrix header for the specified row span.
    /// 
    /// The method makes a new header for the specified row span of the matrix. Similarly to Mat::row and
    /// Mat::col , this is an O(1) operation.
    /// ## Parameters
    /// * startrow: An inclusive 0-based start index of the row span.
    /// * endrow: An exclusive 0-based ending index of the row span.
    pub fn rowbounds(&self, startrow: i32, endrow: i32) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_rowRange_const_int_int(self.as_raw_Mat(), startrow, endrow) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * r: Range structure containing both the start and the end indices.
    pub fn row_range(&self, r: &core::Range) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_rowRange_const_Range(self.as_raw_Mat(), r.as_raw_Range()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Creates a matrix header for the specified column span.
    /// 
    /// The method makes a new header for the specified column span of the matrix. Similarly to Mat::row and
    /// Mat::col , this is an O(1) operation.
    /// ## Parameters
    /// * startcol: An inclusive 0-based start index of the column span.
    /// * endcol: An exclusive 0-based ending index of the column span.
    pub fn colbounds(&self, startcol: i32, endcol: i32) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_colRange_const_int_int(self.as_raw_Mat(), startcol, endcol) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * r: Range structure containing both the start and the end indices.
    pub fn colrange(&self, r: &core::Range) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_colRange_const_Range(self.as_raw_Mat(), r.as_raw_Range()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Extracts a diagonal from a matrix
    /// 
    /// The method makes a new header for the specified matrix diagonal. The new matrix is represented as a
    /// single-column matrix. Similarly to Mat::row and Mat::col, this is an O(1) operation.
    /// ## Parameters
    /// * d: index of the diagonal, with the following values:
    /// - `d=0` is the main diagonal.
    /// - `d<0` is a diagonal from the lower half. For example, d=-1 means the diagonal is set
    /// immediately below the main one.
    /// - `d>0` is a diagonal from the upper half. For example, d=1 means the diagonal is set
    /// immediately above the main one.
    /// For example:
    /// ```ignore
    /// Mat m = (Mat_<int>(3,3) <<
    /// 1,2,3,
    /// 4,5,6,
    /// 7,8,9);
    /// Mat d0 = m.diag(0);
    /// Mat d1 = m.diag(1);
    /// Mat d_1 = m.diag(-1);
    /// ```
    /// 
    /// The resulting matrices are
    /// ```ignore
    /// d0 =
    /// [1;
    /// 5;
    /// 9]
    /// d1 =
    /// [2;
    /// 6]
    /// d_1 =
    /// [4;
    /// 8]
    /// ```
    ///
    /// ## C++ default parameters
    /// * d: 0
    pub fn diag(&self, d: i32) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_diag_const_int(self.as_raw_Mat(), d) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// creates a diagonal matrix
    /// 
    /// The method creates a square diagonal matrix from specified main diagonal.
    /// ## Parameters
    /// * d: One-dimensional matrix that represents the main diagonal.
    pub fn diag_new_mat(d: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_diag_Mat(d.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Creates a full copy of the array and the underlying data.
    /// 
    /// The method creates a full copy of the array. The original step[] is not taken into account. So, the
    /// array copy is a continuous array occupying total()*elemSize() bytes.
    pub fn clone(&self) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_clone_const(self.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Copies the matrix to another one.
    /// 
    /// The method copies the matrix data to another matrix. Before copying the data, the method invokes :
    /// ```ignore
    /// m.create(this->size(), this->type());
    /// ```
    /// 
    /// so that the destination matrix is reallocated if needed. While m.copyTo(m); works flawlessly, the
    /// function does not handle the case of a partial overlap between the source and the destination
    /// matrices.
    /// 
    /// When the operation mask is specified, if the Mat::create call shown above reallocates the matrix,
    /// the newly allocated matrix is initialized with all zeros before copying the data.
    /// ## Parameters
    /// * m: Destination matrix. If it does not have a proper size or type before the operation, it is
    /// reallocated.
    pub fn copy_to(&self, m: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_Mat_copyTo_const_Mat(self.as_raw_Mat(), m.as_raw_Mat()) }.into_result()
    }
    
    /// ## Parameters
    /// * m: Destination matrix. If it does not have a proper size or type before the operation, it is
    /// reallocated.
    /// * mask: Operation mask of the same size as \*this. Its non-zero elements indicate which matrix
    /// elements need to be copied. The mask has to be of type CV_8U and can have 1 or multiple channels.
    pub fn copy_to_masked(&self, m: &mut core::Mat, mask: &core::Mat) -> Result<()> {
        unsafe { sys::cv_Mat_copyTo_const_Mat_Mat(self.as_raw_Mat(), m.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
    }
    
    /// Converts an array to another data type with optional scaling.
    /// 
    /// The method converts source pixel values to the target data type. saturate_cast\<\> is applied at
    /// the end to avoid possible overflows:
    /// 
    /// <div lang='latex'>m(x,y) = saturate \_ cast<rType>( \alpha (*this)(x,y) +  \beta )</div>
    /// ## Parameters
    /// * m: output matrix; if it does not have a proper size or type before the operation, it is
    /// reallocated.
    /// * rtype: desired output matrix type or, rather, the depth since the number of channels are the
    /// same as the input has; if rtype is negative, the output matrix will have the same type as the input.
    /// * alpha: optional scale factor.
    /// * beta: optional delta added to the scaled values.
    ///
    /// ## C++ default parameters
    /// * alpha: 1
    /// * beta: 0
    pub fn convert_to(&self, m: &mut core::Mat, rtype: i32, alpha: f64, beta: f64) -> Result<()> {
        unsafe { sys::cv_Mat_convertTo_const_Mat_int_double_double(self.as_raw_Mat(), m.as_raw_Mat(), rtype, alpha, beta) }.into_result()
    }
    
    /// Provides a functional form of convertTo.
    /// 
    /// This is an internally used method called by the @ref MatrixExpressions engine.
    /// ## Parameters
    /// * m: Destination array.
    /// * type: Desired destination array depth (or -1 if it should be the same as the source type).
    ///
    /// ## C++ default parameters
    /// * _type: -1
    pub fn assign_to(&self, m: &mut core::Mat, _type: i32) -> Result<()> {
        unsafe { sys::cv_Mat_assignTo_const_Mat_int(self.as_raw_Mat(), m.as_raw_Mat(), _type) }.into_result()
    }
    
    /// Sets all or some of the array elements to the specified value.
    /// 
    /// This is an advanced variant of the Mat::operator=(const Scalar& s) operator.
    /// ## Parameters
    /// * value: Assigned scalar converted to the actual array type.
    /// * mask: Operation mask of the same size as \*this. Its non-zero elements indicate which matrix
    /// elements need to be copied. The mask has to be of type CV_8U and can have 1 or multiple channels
    ///
    /// ## C++ default parameters
    /// * mask: noArray()
    pub fn set_to(&mut self, value: &core::Mat, mask: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_setTo_Mat_Mat(self.as_raw_Mat(), value.as_raw_Mat(), mask.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Changes the shape and/or the number of channels of a 2D matrix without copying the data.
    /// 
    /// The method makes a new matrix header for \*this elements. The new matrix may have a different size
    /// and/or different number of channels. Any combination is possible if:
    /// *   No extra elements are included into the new matrix and no elements are excluded. Consequently,
    /// the product rows\*cols\*channels() must stay the same after the transformation.
    /// *   No data is copied. That is, this is an O(1) operation. Consequently, if you change the number of
    /// rows, or the operation changes the indices of elements row in some other way, the matrix must be
    /// continuous. See Mat::isContinuous .
    /// 
    /// For example, if there is a set of 3D points stored as an STL vector, and you want to represent the
    /// points as a 3xN matrix, do the following:
    /// ```ignore
    /// std::vector<Point3f> vec;
    /// ...
    /// Mat pointMat = Mat(vec). // convert vector to Mat, O(1) operation
    /// reshape(1). // make Nx3 1-channel matrix out of Nx1 3-channel.
    /// // Also, an O(1) operation
    /// t(); // finally, transpose the Nx3 matrix.
    /// // This involves copying all the elements
    /// ```
    /// 
    /// ## Parameters
    /// * cn: New number of channels. If the parameter is 0, the number of channels remains the same.
    /// * rows: New number of rows. If the parameter is 0, the number of rows remains the same.
    ///
    /// ## C++ default parameters
    /// * rows: 0
    pub fn reshape(&self, cn: i32, rows: i32) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_reshape_const_int_int(self.as_raw_Mat(), cn, rows) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    pub fn reshape_nd(&self, cn: i32, newshape: &types::VectorOfint) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_reshape_const_int_VectorOfint(self.as_raw_Mat(), cn, newshape.as_raw_VectorOfint()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Computes a cross-product of two 3-element vectors.
    /// 
    /// The method computes a cross-product of two 3-element vectors. The vectors must be 3-element
    /// floating-point vectors of the same shape and size. The result is another 3-element vector of the
    /// same shape and type as operands.
    /// ## Parameters
    /// * m: Another cross-product operand.
    pub fn cross(&self, m: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_cross_const_Mat(self.as_raw_Mat(), m.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Computes a dot-product of two vectors.
    /// 
    /// The method computes a dot-product of two matrices. If the matrices are not single-column or
    /// single-row vectors, the top-to-bottom left-to-right scan ordering is used to treat them as 1D
    /// vectors. The vectors must have the same size and type. If the matrices have more than one channel,
    /// the dot products from all the channels are summed together.
    /// ## Parameters
    /// * m: another dot-product operand.
    pub fn dot(&self, m: &core::Mat) -> Result<f64> {
        unsafe { sys::cv_Mat_dot_const_Mat(self.as_raw_Mat(), m.as_raw_Mat()) }.into_result()
    }
    
    /// Allocates new array data if needed.
    /// 
    /// This is one of the key Mat methods. Most new-style OpenCV functions and methods that produce arrays
    /// call this method for each output array. The method uses the following algorithm:
    /// 
    /// -# If the current array shape and the type match the new ones, return immediately. Otherwise,
    /// de-reference the previous data by calling Mat::release.
    /// -# Initialize the new header.
    /// -# Allocate the new data of total()\*elemSize() bytes.
    /// -# Allocate the new, associated with the data, reference counter and set it to 1.
    /// 
    /// Such a scheme makes the memory management robust and efficient at the same time and helps avoid
    /// extra typing for you. This means that usually there is no need to explicitly allocate output arrays.
    /// That is, instead of writing:
    /// ```ignore
    /// Mat color;
    /// ...
    /// Mat gray(color.rows, color.cols, color.depth());
    /// cvtColor(color, gray, COLOR_BGR2GRAY);
    /// ```
    /// 
    /// you can simply write:
    /// ```ignore
    /// Mat color;
    /// ...
    /// Mat gray;
    /// cvtColor(color, gray, COLOR_BGR2GRAY);
    /// ```
    /// 
    /// because cvtColor, as well as the most of OpenCV functions, calls Mat::create() for the output array
    /// internally.
    /// ## Parameters
    /// * rows: New number of rows.
    /// * cols: New number of columns.
    /// * type: New matrix type.
    pub fn create_rows_cols(&mut self, rows: i32, cols: i32, _type: i32) -> Result<()> {
        unsafe { sys::cv_Mat_create_int_int_int(self.as_raw_Mat(), rows, cols, _type) }.into_result()
    }
    
    /// ## Parameters
    /// * size: Alternative new matrix size specification: Size(cols, rows)
    /// * type: New matrix type.
    pub fn create_size(&mut self, size: core::Size, _type: i32) -> Result<()> {
        unsafe { sys::cv_Mat_create_Size_int(self.as_raw_Mat(), size, _type) }.into_result()
    }
    
    /// ## Parameters
    /// * sizes: Array of integers specifying a new array shape.
    /// * type: New matrix type.
    pub fn create_nd(&mut self, sizes: &types::VectorOfint, _type: i32) -> Result<()> {
        unsafe { sys::cv_Mat_create_VectorOfint_int(self.as_raw_Mat(), sizes.as_raw_VectorOfint(), _type) }.into_result()
    }
    
    /// Increments the reference counter.
    /// 
    /// The method increments the reference counter associated with the matrix data. If the matrix header
    /// points to an external data set (see Mat::Mat ), the reference counter is NULL, and the method has no
    /// effect in this case. Normally, to avoid memory leaks, the method should not be called explicitly. It
    /// is called implicitly by the matrix assignment operator. The reference counter increment is an atomic
    /// operation on the platforms that support it. Thus, it is safe to operate on the same matrices
    /// asynchronously in different threads.
    pub fn addref(&mut self) -> Result<()> {
        unsafe { sys::cv_Mat_addref(self.as_raw_Mat()) }.into_result()
    }
    
    /// Decrements the reference counter and deallocates the matrix if needed.
    /// 
    /// The method decrements the reference counter associated with the matrix data. When the reference
    /// counter reaches 0, the matrix data is deallocated and the data and the reference counter pointers
    /// are set to NULL's. If the matrix header points to an external data set (see Mat::Mat ), the
    /// reference counter is NULL, and the method has no effect in this case.
    /// 
    /// This method can be called manually to force the matrix data deallocation. But since this method is
    /// automatically called in the destructor, or by any other method that changes the data pointer, it is
    /// usually not needed. The reference counter decrement and check for 0 is an atomic operation on the
    /// platforms that support it. Thus, it is safe to operate on the same matrices asynchronously in
    /// different threads.
    pub fn release(&mut self) -> Result<()> {
        unsafe { sys::cv_Mat_release(self.as_raw_Mat()) }.into_result()
    }
    
    /// internal use function, consider to use 'release' method instead; deallocates the matrix data
    pub fn deallocate(&mut self) -> Result<()> {
        unsafe { sys::cv_Mat_deallocate(self.as_raw_Mat()) }.into_result()
    }
    
    /// internal use function; properly re-allocates _size, _step arrays
    pub fn copy_size(&mut self, m: &core::Mat) -> Result<()> {
        unsafe { sys::cv_Mat_copySize_Mat(self.as_raw_Mat(), m.as_raw_Mat()) }.into_result()
    }
    
    /// Reserves space for the certain number of rows.
    /// 
    /// The method reserves space for sz rows. If the matrix already has enough space to store sz rows,
    /// nothing happens. If the matrix is reallocated, the first Mat::rows rows are preserved. The method
    /// emulates the corresponding method of the STL vector class.
    /// ## Parameters
    /// * sz: Number of rows.
    pub fn reserve(&mut self, sz: size_t) -> Result<()> {
        unsafe { sys::cv_Mat_reserve_size_t(self.as_raw_Mat(), sz) }.into_result()
    }
    
    /// Reserves space for the certain number of bytes.
    /// 
    /// The method reserves space for sz bytes. If the matrix already has enough space to store sz bytes,
    /// nothing happens. If matrix has to be reallocated its previous content could be lost.
    /// ## Parameters
    /// * sz: Number of bytes.
    pub fn reserve_buffer(&mut self, sz: size_t) -> Result<()> {
        unsafe { sys::cv_Mat_reserveBuffer_size_t(self.as_raw_Mat(), sz) }.into_result()
    }
    
    /// Changes the number of matrix rows.
    /// 
    /// The methods change the number of matrix rows. If the matrix is reallocated, the first
    /// min(Mat::rows, sz) rows are preserved. The methods emulate the corresponding methods of the STL
    /// vector class.
    /// ## Parameters
    /// * sz: New number of rows.
    pub fn resize(&mut self, sz: size_t) -> Result<()> {
        unsafe { sys::cv_Mat_resize_size_t(self.as_raw_Mat(), sz) }.into_result()
    }
    
    /// ## Parameters
    /// * sz: New number of rows.
    /// * s: Value assigned to the newly added elements.
    pub fn resize_with_default(&mut self, sz: size_t, s: core::Scalar) -> Result<()> {
        unsafe { sys::cv_Mat_resize_size_t_Scalar(self.as_raw_Mat(), sz, s) }.into_result()
    }
    
    /// internal function
    pub fn push_back_(&mut self, elem: &c_void) -> Result<()> {
        unsafe { sys::cv_Mat_push_back__const_void_X(self.as_raw_Mat(), elem) }.into_result()
    }
    
    /// ## Parameters
    /// * m: Added line(s).
    pub fn push_back(&mut self, m: &core::Mat) -> Result<()> {
        unsafe { sys::cv_Mat_push_back_Mat(self.as_raw_Mat(), m.as_raw_Mat()) }.into_result()
    }
    
    /// Removes elements from the bottom of the matrix.
    /// 
    /// The method removes one or more rows from the bottom of the matrix.
    /// ## Parameters
    /// * nelems: Number of removed rows. If it is greater than the total number of rows, an exception
    /// is thrown.
    ///
    /// ## C++ default parameters
    /// * nelems: 1
    pub fn pop_back(&mut self, nelems: size_t) -> Result<()> {
        unsafe { sys::cv_Mat_pop_back_size_t(self.as_raw_Mat(), nelems) }.into_result()
    }
    
    /// Locates the matrix header within a parent matrix.
    /// 
    /// After you extracted a submatrix from a matrix using Mat::row, Mat::col, Mat::rowRange,
    /// Mat::colRange, and others, the resultant submatrix points just to the part of the original big
    /// matrix. However, each submatrix contains information (represented by datastart and dataend
    /// fields) that helps reconstruct the original matrix size and the position of the extracted
    /// submatrix within the original matrix. The method locateROI does exactly that.
    /// ## Parameters
    /// * wholeSize: Output parameter that contains the size of the whole matrix containing *this*
    /// as a part.
    /// * ofs: Output parameter that contains an offset of *this* inside the whole matrix.
    pub fn locate_roi(&self, whole_size: &mut core::Size, ofs: &mut core::Point) -> Result<()> {
        unsafe { sys::cv_Mat_locateROI_const_Size_Point(self.as_raw_Mat(), whole_size, ofs) }.into_result()
    }
    
    /// Adjusts a submatrix size and position within the parent matrix.
    /// 
    /// The method is complimentary to Mat::locateROI . The typical use of these functions is to determine
    /// the submatrix position within the parent matrix and then shift the position somehow. Typically, it
    /// can be required for filtering operations when pixels outside of the ROI should be taken into
    /// account. When all the method parameters are positive, the ROI needs to grow in all directions by the
    /// specified amount, for example:
    /// ```ignore
    /// A.adjustROI(2, 2, 2, 2);
    /// ```
    /// 
    /// In this example, the matrix size is increased by 4 elements in each direction. The matrix is shifted
    /// by 2 elements to the left and 2 elements up, which brings in all the necessary pixels for the
    /// filtering with the 5x5 kernel.
    /// 
    /// adjustROI forces the adjusted ROI to be inside of the parent matrix that is boundaries of the
    /// adjusted ROI are constrained by boundaries of the parent matrix. For example, if the submatrix A is
    /// located in the first row of a parent matrix and you called A.adjustROI(2, 2, 2, 2) then A will not
    /// be increased in the upward direction.
    /// 
    /// The function is used internally by the OpenCV filtering functions, like filter2D , morphological
    /// operations, and so on.
    /// ## Parameters
    /// * dtop: Shift of the top submatrix boundary upwards.
    /// * dbottom: Shift of the bottom submatrix boundary downwards.
    /// * dleft: Shift of the left submatrix boundary to the left.
    /// * dright: Shift of the right submatrix boundary to the right.
    /// ## See also
    /// copyMakeBorder
    pub fn adjust_roi(&mut self, dtop: i32, dbottom: i32, dleft: i32, dright: i32) -> Result<core::Mat> {
        unsafe { sys::cv_Mat_adjustROI_int_int_int_int(self.as_raw_Mat(), dtop, dbottom, dleft, dright) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// Reports whether the matrix is continuous or not.
    /// 
    /// The method returns true if the matrix elements are stored continuously without gaps at the end of
    /// each row. Otherwise, it returns false. Obviously, 1x1 or 1xN matrices are always continuous.
    /// Matrices created with Mat::create are always continuous. But if you extract a part of the matrix
    /// using Mat::col, Mat::diag, and so on, or constructed a matrix header for externally allocated data,
    /// such matrices may no longer have this property.
    /// 
    /// The continuity flag is stored as a bit in the Mat::flags field and is computed automatically when
    /// you construct a matrix header. Thus, the continuity check is a very fast operation, though
    /// theoretically it could be done as follows:
    /// ```ignore
    /// // alternative implementation of Mat::isContinuous()
    /// bool myCheckMatContinuity(const Mat& m)
    /// {
    /// //return (m.flags & Mat::CONTINUOUS_FLAG) != 0;
    /// return m.rows == 1 || m.step == m.cols*m.elemSize();
    /// }
    /// ```
    /// 
    /// The method is used in quite a few of OpenCV functions. The point is that element-wise operations
    /// (such as arithmetic and logical operations, math functions, alpha blending, color space
    /// transformations, and others) do not depend on the image geometry. Thus, if all the input and output
    /// arrays are continuous, the functions can process them as very long single-row vectors. The example
    /// below illustrates how an alpha-blending function can be implemented:
    /// ```ignore
    /// template<typename T>
    /// void alphaBlendRGBA(const Mat& src1, const Mat& src2, Mat& dst)
    /// {
    /// const float alpha_scale = (float)std::numeric_limits<T>::max(),
    /// inv_scale = 1.f/alpha_scale;
    /// 
    /// CV_Assert( src1.type() == src2.type() &&
    /// src1.type() == CV_MAKETYPE(traits::Depth<T>::value, 4) &&
    /// src1.size() == src2.size());
    /// Size size = src1.size();
    /// dst.create(size, src1.type());
    /// 
    /// // here is the idiom: check the arrays for continuity and,
    /// // if this is the case,
    /// // treat the arrays as 1D vectors
    /// if( src1.isContinuous() && src2.isContinuous() && dst.isContinuous() )
    /// {
    /// size.width *= size.height;
    /// size.height = 1;
    /// }
    /// size.width *= 4;
    /// 
    /// for( int i = 0; i < size.height; i++ )
    /// {
    /// // when the arrays are continuous,
    /// // the outer loop is executed only once
    /// const T* ptr1 = src1.ptr<T>(i);
    /// const T* ptr2 = src2.ptr<T>(i);
    /// T* dptr = dst.ptr<T>(i);
    /// 
    /// for( int j = 0; j < size.width; j += 4 )
    /// {
    /// float alpha = ptr1[j+3]*inv_scale, beta = ptr2[j+3]*inv_scale;
    /// dptr[j] = saturate_cast<T>(ptr1[j]*alpha + ptr2[j]*beta);
    /// dptr[j+1] = saturate_cast<T>(ptr1[j+1]*alpha + ptr2[j+1]*beta);
    /// dptr[j+2] = saturate_cast<T>(ptr1[j+2]*alpha + ptr2[j+2]*beta);
    /// dptr[j+3] = saturate_cast<T>((1 - (1-alpha)*(1-beta))*alpha_scale);
    /// }
    /// }
    /// }
    /// ```
    /// 
    /// This approach, while being very simple, can boost the performance of a simple element-operation by
    /// 10-20 percents, especially if the image is rather small and the operation is quite simple.
    /// 
    /// Another OpenCV idiom in this function, a call of Mat::create for the destination array, that
    /// allocates the destination array unless it already has the proper size and type. And while the newly
    /// allocated arrays are always continuous, you still need to check the destination array because
    /// Mat::create does not always allocate a new matrix.
    pub fn is_continuous(&self) -> Result<bool> {
        unsafe { sys::cv_Mat_isContinuous_const(self.as_raw_Mat()) }.into_result()
    }
    
    /// returns true if the matrix is a submatrix of another matrix
    pub fn is_submatrix(&self) -> Result<bool> {
        unsafe { sys::cv_Mat_isSubmatrix_const(self.as_raw_Mat()) }.into_result()
    }
    
    /// Returns the matrix element size in bytes.
    /// 
    /// The method returns the matrix element size in bytes. For example, if the matrix type is CV_16SC3 ,
    /// the method returns 3\*sizeof(short) or 6.
    pub fn elem_size(&self) -> Result<size_t> {
        unsafe { sys::cv_Mat_elemSize_const(self.as_raw_Mat()) }.into_result()
    }
    
    /// Returns the size of each matrix element channel in bytes.
    /// 
    /// The method returns the matrix element channel size in bytes, that is, it ignores the number of
    /// channels. For example, if the matrix type is CV_16SC3 , the method returns sizeof(short) or 2.
    pub fn elem_size1(&self) -> Result<size_t> {
        unsafe { sys::cv_Mat_elemSize1_const(self.as_raw_Mat()) }.into_result()
    }
    
    /// Returns the type of a matrix element.
    /// 
    /// The method returns a matrix element type. This is an identifier compatible with the CvMat type
    /// system, like CV_16SC3 or 16-bit signed 3-channel array, and so on.
    pub fn typ(&self) -> Result<i32> {
        unsafe { sys::cv_Mat_type_const(self.as_raw_Mat()) }.into_result()
    }
    
    /// Returns the depth of a matrix element.
    /// 
    /// The method returns the identifier of the matrix element depth (the type of each individual channel).
    /// For example, for a 16-bit signed element array, the method returns CV_16S . A complete list of
    /// matrix types contains the following values:
    /// *   CV_8U - 8-bit unsigned integers ( 0..255 )
    /// *   CV_8S - 8-bit signed integers ( -128..127 )
    /// *   CV_16U - 16-bit unsigned integers ( 0..65535 )
    /// *   CV_16S - 16-bit signed integers ( -32768..32767 )
    /// *   CV_32S - 32-bit signed integers ( -2147483648..2147483647 )
    /// *   CV_32F - 32-bit floating-point numbers ( -FLT_MAX..FLT_MAX, INF, NAN )
    /// *   CV_64F - 64-bit floating-point numbers ( -DBL_MAX..DBL_MAX, INF, NAN )
    pub fn depth(&self) -> Result<i32> {
        unsafe { sys::cv_Mat_depth_const(self.as_raw_Mat()) }.into_result()
    }
    
    /// Returns the number of matrix channels.
    /// 
    /// The method returns the number of matrix channels.
    pub fn channels(&self) -> Result<i32> {
        unsafe { sys::cv_Mat_channels_const(self.as_raw_Mat()) }.into_result()
    }
    
    /// Returns a normalized step.
    /// 
    /// The method returns a matrix step divided by Mat::elemSize1() . It can be useful to quickly access an
    /// arbitrary matrix element.
    ///
    /// ## C++ default parameters
    /// * i: 0
    pub fn step1(&self, i: i32) -> Result<size_t> {
        unsafe { sys::cv_Mat_step1_const_int(self.as_raw_Mat(), i) }.into_result()
    }
    
    /// Returns true if the array has no elements.
    /// 
    /// The method returns true if Mat::total() is 0 or if Mat::data is NULL. Because of pop_back() and
    /// resize() methods `M.total() == 0` does not imply that `M.data == NULL`.
    pub fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_Mat_empty_const(self.as_raw_Mat()) }.into_result()
    }
    
    /// Returns the total number of array elements.
    /// 
    /// The method returns the number of array elements (a number of pixels if the array represents an
    /// image).
    pub fn total(&self) -> Result<size_t> {
        unsafe { sys::cv_Mat_total_const(self.as_raw_Mat()) }.into_result()
    }
    
    /// Returns the total number of array elements.
    /// 
    /// The method returns the number of elements within a certain sub-array slice with startDim <= dim < endDim
    ///
    /// ## C++ default parameters
    /// * end_dim: INT_MAX
    pub fn total_slice(&self, start_dim: i32, end_dim: i32) -> Result<size_t> {
        unsafe { sys::cv_Mat_total_const_int_int(self.as_raw_Mat(), start_dim, end_dim) }.into_result()
    }
    
    /// ## Parameters
    /// * elemChannels: Number of channels or number of columns the matrix should have.
    ///      For a 2-D matrix, when the matrix has only 1 column, then it should have
    ///      elemChannels channels; When the matrix has only 1 channel,
    ///      then it should have elemChannels columns.
    ///      For a 3-D matrix, it should have only one channel. Furthermore,
    ///      if the number of planes is not one, then the number of rows
    ///      within every plane has to be 1; if the number of rows within
    ///      every plane is not 1, then the number of planes has to be 1.
    /// * depth: The depth the matrix should have. Set it to -1 when any depth is fine.
    /// * requireContinuous: Set it to true to require the matrix to be continuous
    /// ## Returns
    /// -1 if the requirement is not satisfied.
    ///   Otherwise, it returns the number of elements in the matrix. Note
    ///   that an element may have multiple channels.
    /// 
    /// The following code demonstrates its usage for a 2-d matrix:
    /// @snippet snippets/core_mat_checkVector.cpp example-2d
    /// 
    /// The following code demonstrates its usage for a 3-d matrix:
    /// @snippet snippets/core_mat_checkVector.cpp example-3d
    ///
    /// ## C++ default parameters
    /// * depth: -1
    /// * require_continuous: true
    pub fn check_vector(&self, elem_channels: i32, depth: i32, require_continuous: bool) -> Result<i32> {
        unsafe { sys::cv_Mat_checkVector_const_int_int_bool(self.as_raw_Mat(), elem_channels, depth, require_continuous) }.into_result()
    }
    
    /// Returns a pointer to the specified matrix row.
    /// 
    /// The methods return `uchar*` or typed pointer to the specified matrix row. See the sample in
    /// Mat::isContinuous to know how to use these methods.
    /// ## Parameters
    /// * i0: A 0-based row index.
    ///
    /// ## C++ default parameters
    /// * i0: 0
    pub fn ptr_mut(&mut self, i0: i32) -> Result<&mut u8> {
        unsafe { sys::cv_Mat_ptr_int(self.as_raw_Mat(), i0) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    ///
    /// ## C++ default parameters
    /// * i0: 0
    pub fn ptr(&self, i0: i32) -> Result<&u8> {
        unsafe { sys::cv_Mat_ptr_const_int(self.as_raw_Mat(), i0) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    /// ## Parameters
    /// * row: Index along the dimension 0
    /// * col: Index along the dimension 1
    pub fn ptr_2d_mut(&mut self, row: i32, col: i32) -> Result<&mut u8> {
        unsafe { sys::cv_Mat_ptr_int_int(self.as_raw_Mat(), row, col) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    /// ## Parameters
    /// * row: Index along the dimension 0
    /// * col: Index along the dimension 1
    pub fn ptr_2d(&self, row: i32, col: i32) -> Result<&u8> {
        unsafe { sys::cv_Mat_ptr_const_int_int(self.as_raw_Mat(), row, col) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    pub fn ptr_3d_mut(&mut self, i0: i32, i1: i32, i2: i32) -> Result<&mut u8> {
        unsafe { sys::cv_Mat_ptr_int_int_int(self.as_raw_Mat(), i0, i1, i2) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    pub fn ptr_3d(&self, i0: i32, i1: i32, i2: i32) -> Result<&u8> {
        unsafe { sys::cv_Mat_ptr_const_int_int_int(self.as_raw_Mat(), i0, i1, i2) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    pub fn ptr_nd_mut(&mut self, idx: &i32) -> Result<&mut u8> {
        unsafe { sys::cv_Mat_ptr_const_int_X(self.as_raw_Mat(), idx) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    pub fn ptr_nd(&self, idx: &i32) -> Result<&u8> {
        unsafe { sys::cv_Mat_ptr_const_const_int_X(self.as_raw_Mat(), idx) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    /// Returns a reference to the specified array element.
    /// 
    /// The template methods return a reference to the specified array element. For the sake of higher
    /// performance, the index range checks are only performed in the Debug configuration.
    /// 
    /// Note that the variants with a single index (i) can be used to access elements of single-row or
    /// single-column 2-dimensional arrays. That is, if, for example, A is a 1 x N floating-point matrix and
    /// B is an M x 1 integer matrix, you can simply write `A.at<float>(k+4)` and `B.at<int>(2*i+1)`
    /// instead of `A.at<float>(0,k+4)` and `B.at<int>(2*i+1,0)`, respectively.
    /// 
    /// The example below initializes a Hilbert matrix:
    /// ```ignore
    /// Mat H(100, 100, CV_64F);
    /// for(int i = 0; i < H.rows; i++)
    /// for(int j = 0; j < H.cols; j++)
    /// H.at<double>(i,j)=1./(i+j+1);
    /// ```
    /// 
    /// 
    /// Keep in mind that the size identifier used in the at operator cannot be chosen at random. It depends
    /// on the image from which you are trying to retrieve the data. The table below gives a better insight in this:
    /// - If matrix is of type `CV_8U` then use `Mat.at<uchar>(y,x)`.
    /// - If matrix is of type `CV_8S` then use `Mat.at<schar>(y,x)`.
    /// - If matrix is of type `CV_16U` then use `Mat.at<ushort>(y,x)`.
    /// - If matrix is of type `CV_16S` then use `Mat.at<short>(y,x)`.
    /// - If matrix is of type `CV_32S`  then use `Mat.at<int>(y,x)`.
    /// - If matrix is of type `CV_32F`  then use `Mat.at<float>(y,x)`.
    /// - If matrix is of type `CV_64F` then use `Mat.at<double>(y,x)`.
    /// 
    /// ## Parameters
    /// * i0: Index along the dimension 0
    ///
    /// ## C++ default parameters
    /// * i0: 0
    pub fn at_mut<T: core::DataType>(&mut self, i0: i32) -> Result<&mut T> { self._at_mut(i0) }
    
    /// ## Parameters
    /// * i0: Index along the dimension 0
    ///
    /// ## C++ default parameters
    /// * i0: 0
    pub fn at<T: core::DataType>(&self, i0: i32) -> Result<&T> { self._at(i0) }
    
    /// ## Parameters
    /// * row: Index along the dimension 0
    /// * col: Index along the dimension 1
    pub fn at_2d_mut<T: core::DataType>(&mut self, row: i32, col: i32) -> Result<&mut T> { self._at_2d_mut(row, col) }
    
    /// ## Parameters
    /// * row: Index along the dimension 0
    /// * col: Index along the dimension 1
    pub fn at_2d<T: core::DataType>(&self, row: i32, col: i32) -> Result<&T> { self._at_2d(row, col) }
    
    /// ## Parameters
    /// * i0: Index along the dimension 0
    /// * i1: Index along the dimension 1
    /// * i2: Index along the dimension 2
    pub fn at_3d_mut<T: core::DataType>(&mut self, i0: i32, i1: i32, i2: i32) -> Result<&mut T> { self._at_3d_mut(i0, i1, i2) }
    
    /// ## Parameters
    /// * i0: Index along the dimension 0
    /// * i1: Index along the dimension 1
    /// * i2: Index along the dimension 2
    pub fn at_3d<T: core::DataType>(&self, i0: i32, i1: i32, i2: i32) -> Result<&T> { self._at_3d(i0, i1, i2) }
    
    /// internal use method: updates the continuity flag
    pub fn update_continuity_flag(&mut self) -> Result<()> {
        unsafe { sys::cv_Mat_updateContinuityFlag(self.as_raw_Mat()) }.into_result()
    }
    
    pub fn size(&self) -> Result<core::Size> {
        unsafe { sys::cv_Mat_size_const(self.as_raw_Mat()) }.into_result()
    }
    
    pub fn flags(&self) -> Result<i32> {
        unsafe { sys::cv_Mat_flags_const(self.as_raw_Mat()) }.into_result()
    }
    
    /// the matrix dimensionality, >= 2
    pub fn dims(&self) -> Result<i32> {
        unsafe { sys::cv_Mat_dims_const(self.as_raw_Mat()) }.into_result()
    }
    
    /// the number of rows and columns or (-1, -1) when the matrix has more than 2 dimensions
    pub fn rows(&self) -> Result<i32> {
        unsafe { sys::cv_Mat_rows_const(self.as_raw_Mat()) }.into_result()
    }
    
    /// the number of rows and columns or (-1, -1) when the matrix has more than 2 dimensions
    pub fn cols(&self) -> Result<i32> {
        unsafe { sys::cv_Mat_cols_const(self.as_raw_Mat()) }.into_result()
    }
    
    /// pointer to the data
    pub fn data(&mut self) -> Result<&mut u8> {
        unsafe { sys::cv_Mat_data(self.as_raw_Mat()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    /// pointer to the data
    pub unsafe fn set_data(&mut self, val: &mut u8) -> Result<()> {
        { sys::cv_Mat_set_data_uchar_X(self.as_raw_Mat(), val) }.into_result()
    }
    
    /// helper fields used in locateROI and adjustROI
    pub fn datastart(&self) -> Result<&u8> {
        unsafe { sys::cv_Mat_datastart_const(self.as_raw_Mat()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    pub fn dataend(&self) -> Result<&u8> {
        unsafe { sys::cv_Mat_dataend_const(self.as_raw_Mat()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    pub fn datalimit(&self) -> Result<&u8> {
        unsafe { sys::cv_Mat_datalimit_const(self.as_raw_Mat()) }.into_result().and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    pub fn step(&mut self) -> Result<core::MatStep> {
        unsafe { sys::cv_Mat_step(self.as_raw_Mat()) }.into_result().map(|x| core::MatStep { ptr: x })
    }
    
}

// boxed class cv::MatExpr
/// Matrix expression representation
/// @anchor MatrixExpressions
/// This is a list of implemented matrix operations that can be combined in arbitrary complex
/// expressions (here A, B stand for matrices ( Mat ), s for a scalar ( Scalar ), alpha for a
/// real-valued scalar ( double )):
/// *   Addition, subtraction, negation: `A+B`, `A-B`, `A+s`, `A-s`, `s+A`, `s-A`, `-A`
/// *   Scaling: `A*alpha`
/// *   Per-element multiplication and division: `A.mul(B)`, `A/B`, `alpha/A`
/// *   Matrix multiplication: `A*B`
/// *   Transposition: `A.t()` (means A<sup>T</sup>)
/// *   Matrix inversion and pseudo-inversion, solving linear systems and least-squares problems:
/// `A.inv([method]) (~ A<sup>-1</sup>)`,   `A.inv([method])*B (~ X: AX=B)`
/// *   Comparison: `A cmpop B`, `A cmpop alpha`, `alpha cmpop A`, where *cmpop* is one of
/// `>`, `>=`, `==`, `!=`, `<=`, `<`. The result of comparison is an 8-bit single channel mask whose
/// elements are set to 255 (if the particular element or pair of elements satisfy the condition) or
/// 0.
/// *   Bitwise logical operations: `A logicop B`, `A logicop s`, `s logicop A`, `~A`, where *logicop* is one of
/// `&`, `|`, `^`.
/// *   Element-wise minimum and maximum: `min(A, B)`, `min(A, alpha)`, `max(A, B)`, `max(A, alpha)`
/// *   Element-wise absolute value: `abs(A)`
/// *   Cross-product, dot-product: `A.cross(B)`, `A.dot(B)`
/// *   Any function of matrix or matrices and scalars that returns a matrix or a scalar, such as norm,
/// mean, sum, countNonZero, trace, determinant, repeat, and others.
/// *   Matrix initializers ( Mat::eye(), Mat::zeros(), Mat::ones() ), matrix comma-separated
/// initializers, matrix constructors and operators that extract sub-matrices (see Mat description).
/// *   Mat_<destination_type>() constructors to cast the result to the proper type.
/// 
/// Note: Comma-separated initializers and probably some other operations may require additional
/// explicit Mat() or Mat_<T>() constructor calls to resolve a possible ambiguity.
/// 
/// Here are examples of matrix expressions:
/// ```ignore
/// // compute pseudo-inverse of A, equivalent to A.inv(DECOMP_SVD)
/// SVD svd(A);
/// Mat pinvA = svd.vt.t()*Mat::diag(1./svd.w)*svd.u.t();
/// 
/// // compute the new vector of parameters in the Levenberg-Marquardt algorithm
/// x -= (A.t()*A + lambda*Mat::eye(A.cols,A.cols,A.type())).inv(DECOMP_CHOLESKY)*(A.t()*err);
/// 
/// // sharpen image using "unsharp mask" algorithm
/// Mat blurred; double sigma = 1, threshold = 5, amount = 1;
/// GaussianBlur(img, blurred, Size(), sigma, sigma);
/// Mat lowContrastMask = abs(img - blurred) < threshold;
/// Mat sharpened = img*(1+amount) + blurred*(-amount);
/// img.copyTo(sharpened, lowContrastMask);
/// ```
#[allow(dead_code)]
pub struct MatExpr {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::MatExpr {
    fn drop(&mut self) {
        unsafe { sys::cv_MatExpr_delete(self.ptr) };
    }
}
impl core::MatExpr {
    pub fn as_raw_MatExpr(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        MatExpr {
            ptr
        }
    }
}

impl MatExpr {

    pub fn size(&self) -> Result<core::Size> {
        unsafe { sys::cv_MatExpr_size_const(self.as_raw_MatExpr()) }.into_result()
    }
    
    pub fn typ(&self) -> Result<i32> {
        unsafe { sys::cv_MatExpr_type_const(self.as_raw_MatExpr()) }.into_result()
    }
    
    pub fn cross(&self, m: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_MatExpr_cross_const_Mat(self.as_raw_MatExpr(), m.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    pub fn dot(&self, m: &core::Mat) -> Result<f64> {
        unsafe { sys::cv_MatExpr_dot_const_Mat(self.as_raw_MatExpr(), m.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::MatOp (trait)
pub trait MatOp {
    #[doc(hidden)] fn as_raw_MatOp(&self) -> *mut c_void;
}

impl<'a> MatOp + 'a {

}

// boxed class cv::MatSize
#[allow(dead_code)]
pub struct MatSize {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::MatSize {
    fn drop(&mut self) {
        unsafe { sys::cv_MatSize_delete(self.ptr) };
    }
}
impl core::MatSize {
    pub fn as_raw_MatSize(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        MatSize {
            ptr
        }
    }
}

impl MatSize {

    pub fn new(_p: &mut i32) -> Result<core::MatSize> {
        unsafe { sys::cv_MatSize_MatSize_int_X(_p) }.into_result().map(|x| core::MatSize { ptr: x })
    }
    
    pub fn dims(&self) -> Result<i32> {
        unsafe { sys::cv_MatSize_dims_const(self.as_raw_MatSize()) }.into_result()
    }
    
}

// boxed class cv::MatStep
#[allow(dead_code)]
pub struct MatStep {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::MatStep {
    fn drop(&mut self) {
        unsafe { sys::cv_MatStep_delete(self.ptr) };
    }
}
impl core::MatStep {
    pub fn as_raw_MatStep(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        MatStep {
            ptr
        }
    }
}

impl MatStep {

    pub fn default() -> Result<core::MatStep> {
        unsafe { sys::cv_MatStep_MatStep() }.into_result().map(|x| core::MatStep { ptr: x })
    }
    
    pub fn new(s: size_t) -> Result<core::MatStep> {
        unsafe { sys::cv_MatStep_MatStep_size_t(s) }.into_result().map(|x| core::MatStep { ptr: x })
    }
    
}

// boxed class cv::Matx_AddOp
/// @cond IGNORED
#[allow(dead_code)]
pub struct Matx_AddOp {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::Matx_AddOp {
    fn drop(&mut self) {
        unsafe { sys::cv_Matx_AddOp_delete(self.ptr) };
    }
}
impl core::Matx_AddOp {
    pub fn as_raw_Matx_AddOp(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Matx_AddOp {
            ptr
        }
    }
}

impl Matx_AddOp {

    pub fn new() -> Result<core::Matx_AddOp> {
        unsafe { sys::cv_Matx_AddOp_Matx_AddOp() }.into_result().map(|x| core::Matx_AddOp { ptr: x })
    }
    
    pub fn copy(unnamed_arg: &core::Matx_AddOp) -> Result<core::Matx_AddOp> {
        unsafe { sys::cv_Matx_AddOp_Matx_AddOp_Matx_AddOp(unnamed_arg.as_raw_Matx_AddOp()) }.into_result().map(|x| core::Matx_AddOp { ptr: x })
    }
    
}

// boxed class cv::Matx_DivOp
#[allow(dead_code)]
pub struct Matx_DivOp {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::Matx_DivOp {
    fn drop(&mut self) {
        unsafe { sys::cv_Matx_DivOp_delete(self.ptr) };
    }
}
impl core::Matx_DivOp {
    pub fn as_raw_Matx_DivOp(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Matx_DivOp {
            ptr
        }
    }
}

impl Matx_DivOp {

    pub fn new() -> Result<core::Matx_DivOp> {
        unsafe { sys::cv_Matx_DivOp_Matx_DivOp() }.into_result().map(|x| core::Matx_DivOp { ptr: x })
    }
    
    pub fn copy(unnamed_arg: &core::Matx_DivOp) -> Result<core::Matx_DivOp> {
        unsafe { sys::cv_Matx_DivOp_Matx_DivOp_Matx_DivOp(unnamed_arg.as_raw_Matx_DivOp()) }.into_result().map(|x| core::Matx_DivOp { ptr: x })
    }
    
}

// boxed class cv::Matx_MatMulOp
#[allow(dead_code)]
pub struct Matx_MatMulOp {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::Matx_MatMulOp {
    fn drop(&mut self) {
        unsafe { sys::cv_Matx_MatMulOp_delete(self.ptr) };
    }
}
impl core::Matx_MatMulOp {
    pub fn as_raw_Matx_MatMulOp(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Matx_MatMulOp {
            ptr
        }
    }
}

impl Matx_MatMulOp {

    pub fn new() -> Result<core::Matx_MatMulOp> {
        unsafe { sys::cv_Matx_MatMulOp_Matx_MatMulOp() }.into_result().map(|x| core::Matx_MatMulOp { ptr: x })
    }
    
    pub fn copy(unnamed_arg: &core::Matx_MatMulOp) -> Result<core::Matx_MatMulOp> {
        unsafe { sys::cv_Matx_MatMulOp_Matx_MatMulOp_Matx_MatMulOp(unnamed_arg.as_raw_Matx_MatMulOp()) }.into_result().map(|x| core::Matx_MatMulOp { ptr: x })
    }
    
}

// boxed class cv::Matx_MulOp
#[allow(dead_code)]
pub struct Matx_MulOp {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::Matx_MulOp {
    fn drop(&mut self) {
        unsafe { sys::cv_Matx_MulOp_delete(self.ptr) };
    }
}
impl core::Matx_MulOp {
    pub fn as_raw_Matx_MulOp(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Matx_MulOp {
            ptr
        }
    }
}

impl Matx_MulOp {

    pub fn new() -> Result<core::Matx_MulOp> {
        unsafe { sys::cv_Matx_MulOp_Matx_MulOp() }.into_result().map(|x| core::Matx_MulOp { ptr: x })
    }
    
    pub fn copy(unnamed_arg: &core::Matx_MulOp) -> Result<core::Matx_MulOp> {
        unsafe { sys::cv_Matx_MulOp_Matx_MulOp_Matx_MulOp(unnamed_arg.as_raw_Matx_MulOp()) }.into_result().map(|x| core::Matx_MulOp { ptr: x })
    }
    
}

// boxed class cv::Matx_ScaleOp
#[allow(dead_code)]
pub struct Matx_ScaleOp {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::Matx_ScaleOp {
    fn drop(&mut self) {
        unsafe { sys::cv_Matx_ScaleOp_delete(self.ptr) };
    }
}
impl core::Matx_ScaleOp {
    pub fn as_raw_Matx_ScaleOp(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Matx_ScaleOp {
            ptr
        }
    }
}

impl Matx_ScaleOp {

    pub fn new() -> Result<core::Matx_ScaleOp> {
        unsafe { sys::cv_Matx_ScaleOp_Matx_ScaleOp() }.into_result().map(|x| core::Matx_ScaleOp { ptr: x })
    }
    
    pub fn copy(unnamed_arg: &core::Matx_ScaleOp) -> Result<core::Matx_ScaleOp> {
        unsafe { sys::cv_Matx_ScaleOp_Matx_ScaleOp_Matx_ScaleOp(unnamed_arg.as_raw_Matx_ScaleOp()) }.into_result().map(|x| core::Matx_ScaleOp { ptr: x })
    }
    
}

// boxed class cv::Matx_SubOp
#[allow(dead_code)]
pub struct Matx_SubOp {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::Matx_SubOp {
    fn drop(&mut self) {
        unsafe { sys::cv_Matx_SubOp_delete(self.ptr) };
    }
}
impl core::Matx_SubOp {
    pub fn as_raw_Matx_SubOp(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Matx_SubOp {
            ptr
        }
    }
}

impl Matx_SubOp {

    pub fn new() -> Result<core::Matx_SubOp> {
        unsafe { sys::cv_Matx_SubOp_Matx_SubOp() }.into_result().map(|x| core::Matx_SubOp { ptr: x })
    }
    
    pub fn copy(unnamed_arg: &core::Matx_SubOp) -> Result<core::Matx_SubOp> {
        unsafe { sys::cv_Matx_SubOp_Matx_SubOp_Matx_SubOp(unnamed_arg.as_raw_Matx_SubOp()) }.into_result().map(|x| core::Matx_SubOp { ptr: x })
    }
    
}

// boxed class cv::Matx_TOp
#[allow(dead_code)]
pub struct Matx_TOp {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::Matx_TOp {
    fn drop(&mut self) {
        unsafe { sys::cv_Matx_TOp_delete(self.ptr) };
    }
}
impl core::Matx_TOp {
    pub fn as_raw_Matx_TOp(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Matx_TOp {
            ptr
        }
    }
}

impl Matx_TOp {

    pub fn new() -> Result<core::Matx_TOp> {
        unsafe { sys::cv_Matx_TOp_Matx_TOp() }.into_result().map(|x| core::Matx_TOp { ptr: x })
    }
    
    pub fn copy(unnamed_arg: &core::Matx_TOp) -> Result<core::Matx_TOp> {
        unsafe { sys::cv_Matx_TOp_Matx_TOp_Matx_TOp(unnamed_arg.as_raw_Matx_TOp()) }.into_result().map(|x| core::Matx_TOp { ptr: x })
    }
    
}

// Generating impl for trait cv::MinProblemSolver (trait)
/// Basic interface for all solvers
pub trait MinProblemSolver : core::Algorithm {
    #[doc(hidden)] fn as_raw_MinProblemSolver(&self) -> *mut c_void;
    /// Getter for the optimized function.
    /// 
    /// The optimized function is represented by Function interface, which requires derivatives to
    /// implement the calc(double*) and getDim() methods to evaluate the function.
    /// 
    /// ## Returns
    /// Smart-pointer to an object that implements Function interface - it represents the
    /// function that is being optimized. It can be empty, if no function was given so far.
    fn get_function(&self) -> Result<types::PtrOfFunction> {
        unsafe { sys::cv_MinProblemSolver_getFunction_const(self.as_raw_MinProblemSolver()) }.into_result().map(|x| types::PtrOfFunction { ptr: x })
    }
    
    /// Setter for the optimized function.
    /// 
    /// *It should be called at least once before the call to* minimize(), as default value is not usable.
    /// 
    /// ## Parameters
    /// * f: The new function to optimize.
    fn set_function(&mut self, f: &types::PtrOfFunction) -> Result<()> {
        unsafe { sys::cv_MinProblemSolver_setFunction_PtrOfFunction(self.as_raw_MinProblemSolver(), f.as_raw_PtrOfFunction()) }.into_result()
    }
    
    /// Getter for the previously set terminal criteria for this algorithm.
    /// 
    /// ## Returns
    /// Deep copy of the terminal criteria used at the moment.
    fn get_term_criteria(&self) -> Result<core::TermCriteria> {
        unsafe { sys::cv_MinProblemSolver_getTermCriteria_const(self.as_raw_MinProblemSolver()) }.into_result().map(|x| core::TermCriteria { ptr: x })
    }
    
    /// Set terminal criteria for solver.
    /// 
    /// This method *is not necessary* to be called before the first call to minimize(), as the default
    /// value is sensible.
    /// 
    /// Algorithm stops when the number of function evaluations done exceeds termcrit.maxCount, when
    /// the function values at the vertices of simplex are within termcrit.epsilon range or simplex
    /// becomes so small that it can enclosed in a box with termcrit.epsilon sides, whatever comes
    /// first.
    /// ## Parameters
    /// * termcrit: Terminal criteria to be used, represented as cv::TermCriteria structure.
    fn set_term_criteria(&mut self, termcrit: &core::TermCriteria) -> Result<()> {
        unsafe { sys::cv_MinProblemSolver_setTermCriteria_TermCriteria(self.as_raw_MinProblemSolver(), termcrit.as_raw_TermCriteria()) }.into_result()
    }
    
    /// actually runs the algorithm and performs the minimization.
    /// 
    /// The sole input parameter determines the centroid of the starting simplex (roughly, it tells
    /// where to start), all the others (terminal criteria, initial step, function to be minimized) are
    /// supposed to be set via the setters before the call to this method or the default values (not
    /// always sensible) will be used.
    /// 
    /// ## Parameters
    /// * x: The initial point, that will become a centroid of an initial simplex. After the algorithm
    /// will terminate, it will be set to the point where the algorithm stops, the point of possible
    /// minimum.
    /// ## Returns
    /// The value of a function at the point found.
    fn minimize(&mut self, x: &mut core::Mat) -> Result<f64> {
        unsafe { sys::cv_MinProblemSolver_minimize_Mat(self.as_raw_MinProblemSolver(), x.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> MinProblemSolver + 'a {

}

// Generating impl for trait cv::MinProblemSolver::Function (trait)
/// Represents function being optimized
pub trait MinProblemSolver_Function {
    #[doc(hidden)] fn as_raw_MinProblemSolver_Function(&self) -> *mut c_void;
    fn get_dims(&self) -> Result<i32> {
        unsafe { sys::cv_MinProblemSolver_Function_getDims_const(self.as_raw_MinProblemSolver_Function()) }.into_result()
    }
    
    fn get_gradient_eps(&self) -> Result<f64> {
        unsafe { sys::cv_MinProblemSolver_Function_getGradientEps_const(self.as_raw_MinProblemSolver_Function()) }.into_result()
    }
    
    fn calc(&self, x: &f64) -> Result<f64> {
        unsafe { sys::cv_MinProblemSolver_Function_calc_const_const_double_X(self.as_raw_MinProblemSolver_Function(), x) }.into_result()
    }
    
    fn get_gradient(&mut self, x: &f64, grad: &mut f64) -> Result<()> {
        unsafe { sys::cv_MinProblemSolver_Function_getGradient_const_double_X_double_X(self.as_raw_MinProblemSolver_Function(), x, grad) }.into_result()
    }
    
}

impl<'a> MinProblemSolver_Function + 'a {

}

impl Moments {

    /// the default constructor
    pub fn default() -> Result<core::Moments> {
        unsafe { sys::cv_Moments_Moments() }.into_result()
    }
    
    /// the full constructor
    pub fn new(m00: f64, m10: f64, m01: f64, m20: f64, m11: f64, m02: f64, m30: f64, m21: f64, m12: f64, m03: f64) -> Result<core::Moments> {
        unsafe { sys::cv_Moments_Moments_double_double_double_double_double_double_double_double_double_double(m00, m10, m01, m20, m11, m02, m30, m21, m12, m03) }.into_result()
    }
    
}

// boxed class cv::NAryMatIterator
/// n-ary multi-dimensional array iterator.
/// 
/// Use the class to implement unary, binary, and, generally, n-ary element-wise operations on
/// multi-dimensional arrays. Some of the arguments of an n-ary function may be continuous arrays, some
/// may be not. It is possible to use conventional MatIterator 's for each array but incrementing all of
/// the iterators after each small operations may be a big overhead. In this case consider using
/// NAryMatIterator to iterate through several matrices simultaneously as long as they have the same
/// geometry (dimensionality and all the dimension sizes are the same). On each iteration `it.planes[0]`,
/// `it.planes[1]`,... will be the slices of the corresponding matrices.
/// 
/// The example below illustrates how you can compute a normalized and threshold 3D color histogram:
/// ```ignore
/// void computeNormalizedColorHist(const Mat& image, Mat& hist, int N, double minProb)
/// {
/// const int histSize[] = {N, N, N};
/// 
/// // make sure that the histogram has a proper size and type
/// hist.create(3, histSize, CV_32F);
/// 
/// // and clear it
/// hist = Scalar(0);
/// 
/// // the loop below assumes that the image
/// // is a 8-bit 3-channel. check it.
/// CV_Assert(image.type() == CV_8UC3);
/// MatConstIterator_<Vec3b> it = image.begin<Vec3b>(),
/// it_end = image.end<Vec3b>();
/// for( ; it != it_end; ++it )
/// {
/// const Vec3b& pix = *it;
/// hist.at<float>(pix[0]*N/256, pix[1]*N/256, pix[2]*N/256) += 1.f;
/// }
/// 
/// minProb *= image.rows*image.cols;
/// 
/// // initialize iterator (the style is different from STL).
/// // after initialization the iterator will contain
/// // the number of slices or planes the iterator will go through.
/// // it simultaneously increments iterators for several matrices
/// // supplied as a null terminated list of pointers
/// const Mat* arrays[] = {&hist, 0};
/// Mat planes[1];
/// NAryMatIterator itNAry(arrays, planes, 1);
/// double s = 0;
/// // iterate through the matrix. on each iteration
/// // itNAry.planes[i] (of type Mat) will be set to the current plane
/// // of the i-th n-dim matrix passed to the iterator constructor.
/// for(int p = 0; p < itNAry.nplanes; p++, ++itNAry)
/// {
/// threshold(itNAry.planes[0], itNAry.planes[0], minProb, 0, THRESH_TOZERO);
/// s += sum(itNAry.planes[0])[0];
/// }
/// 
/// s = 1./s;
/// itNAry = NAryMatIterator(arrays, planes, 1);
/// for(int p = 0; p < itNAry.nplanes; p++, ++itNAry)
/// itNAry.planes[0] *= s;
/// }
/// ```
#[allow(dead_code)]
pub struct NAryMatIterator {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::NAryMatIterator {
    fn drop(&mut self) {
        unsafe { sys::cv_NAryMatIterator_delete(self.ptr) };
    }
}
impl core::NAryMatIterator {
    pub fn as_raw_NAryMatIterator(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        NAryMatIterator {
            ptr
        }
    }
}

impl NAryMatIterator {

    /// the default constructor
    pub fn new() -> Result<core::NAryMatIterator> {
        unsafe { sys::cv_NAryMatIterator_NAryMatIterator() }.into_result().map(|x| core::NAryMatIterator { ptr: x })
    }
    
}

// boxed class cv::PCA
/// Principal Component Analysis
/// 
/// The class is used to calculate a special basis for a set of vectors. The
/// basis will consist of eigenvectors of the covariance matrix calculated
/// from the input set of vectors. The class %PCA can also transform
/// vectors to/from the new coordinate space defined by the basis. Usually,
/// in this new coordinate system, each vector from the original set (and
/// any linear combination of such vectors) can be quite accurately
/// approximated by taking its first few components, corresponding to the
/// eigenvectors of the largest eigenvalues of the covariance matrix.
/// Geometrically it means that you calculate a projection of the vector to
/// a subspace formed by a few eigenvectors corresponding to the dominant
/// eigenvalues of the covariance matrix. And usually such a projection is
/// very close to the original vector. So, you can represent the original
/// vector from a high-dimensional space with a much shorter vector
/// consisting of the projected vector's coordinates in the subspace. Such a
/// transformation is also known as Karhunen-Loeve Transform, or KLT.
/// See http://en.wikipedia.org/wiki/Principal_component_analysis
/// 
/// The sample below is the function that takes two matrices. The first
/// function stores a set of vectors (a row per vector) that is used to
/// calculate PCA. The second function stores another "test" set of vectors
/// (a row per vector). First, these vectors are compressed with PCA, then
/// reconstructed back, and then the reconstruction error norm is computed
/// and printed for each vector. :
/// 
/// ```ignore{.cpp}
/// using namespace cv;
/// 
/// PCA compressPCA(const Mat& pcaset, int maxComponents,
/// const Mat& testset, Mat& compressed)
/// {
/// PCA pca(pcaset, // pass the data
/// Mat(), // we do not have a pre-computed mean vector,
/// // so let the PCA engine to compute it
/// PCA::DATA_AS_ROW, // indicate that the vectors
/// // are stored as matrix rows
/// // (use PCA::DATA_AS_COL if the vectors are
/// // the matrix columns)
/// maxComponents // specify, how many principal components to retain
/// );
/// // if there is no test data, just return the computed basis, ready-to-use
/// if( !testset.data )
/// return pca;
/// CV_Assert( testset.cols == pcaset.cols );
/// 
/// compressed.create(testset.rows, maxComponents, testset.type());
/// 
/// Mat reconstructed;
/// for( int i = 0; i < testset.rows; i++ )
/// {
/// Mat vec = testset.row(i), coeffs = compressed.row(i), reconstructed;
/// // compress the vector, the result will be stored
/// // in the i-th row of the output matrix
/// pca.project(vec, coeffs);
/// // and then reconstruct it
/// pca.backProject(coeffs, reconstructed);
/// // and measure the error
/// printf("%d. diff = %g\n", i, norm(vec, reconstructed, NORM_L2));
/// }
/// return pca;
/// }
/// ```
/// 
/// ## See also
/// calcCovarMatrix, mulTransposed, SVD, dft, dct
#[allow(dead_code)]
pub struct PCA {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::PCA {
    fn drop(&mut self) {
        unsafe { sys::cv_PCA_delete(self.ptr) };
    }
}
impl core::PCA {
    pub fn as_raw_PCA(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        PCA {
            ptr
        }
    }
}

impl PCA {

    /// default constructor
    /// 
    /// The default constructor initializes an empty %PCA structure. The other
    /// constructors initialize the structure and call PCA::operator()().
    pub fn default() -> Result<core::PCA> {
        unsafe { sys::cv_PCA_PCA() }.into_result().map(|x| core::PCA { ptr: x })
    }
    
    /// ## Parameters
    /// * data: input samples stored as matrix rows or matrix columns.
    /// * mean: optional mean value; if the matrix is empty (@c noArray()),
    /// the mean is computed from the data.
    /// * flags: operation flags; currently the parameter is only used to
    /// specify the data layout (PCA::Flags)
    /// * maxComponents: maximum number of components that %PCA should
    /// retain; by default, all the components are retained.
    ///
    /// ## C++ default parameters
    /// * max_components: 0
    pub fn new_mat_max(data: &core::Mat, mean: &core::Mat, flags: i32, max_components: i32) -> Result<core::PCA> {
        unsafe { sys::cv_PCA_PCA_Mat_Mat_int_int(data.as_raw_Mat(), mean.as_raw_Mat(), flags, max_components) }.into_result().map(|x| core::PCA { ptr: x })
    }
    
    /// ## Parameters
    /// * data: input samples stored as matrix rows or matrix columns.
    /// * mean: optional mean value; if the matrix is empty (noArray()),
    /// the mean is computed from the data.
    /// * flags: operation flags; currently the parameter is only used to
    /// specify the data layout (PCA::Flags)
    /// * retainedVariance: Percentage of variance that PCA should retain.
    /// Using this parameter will let the PCA decided how many components to
    /// retain but it will always keep at least 2.
    pub fn new_mat_variance(data: &core::Mat, mean: &core::Mat, flags: i32, retained_variance: f64) -> Result<core::PCA> {
        unsafe { sys::cv_PCA_PCA_Mat_Mat_int_double(data.as_raw_Mat(), mean.as_raw_Mat(), flags, retained_variance) }.into_result().map(|x| core::PCA { ptr: x })
    }
    
    /// Projects vector(s) to the principal component subspace.
    /// 
    /// The methods project one or more vectors to the principal component
    /// subspace, where each vector projection is represented by coefficients in
    /// the principal component basis. The first form of the method returns the
    /// matrix that the second form writes to the result. So the first form can
    /// be used as a part of expression while the second form can be more
    /// efficient in a processing loop.
    /// ## Parameters
    /// * vec: input vector(s); must have the same dimensionality and the
    /// same layout as the input data used at %PCA phase, that is, if
    /// DATA_AS_ROW are specified, then `vec.cols==data.cols`
    /// (vector dimensionality) and `vec.rows` is the number of vectors to
    /// project, and the same is true for the PCA::DATA_AS_COL case.
    pub fn project(&self, vec: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_PCA_project_const_Mat(self.as_raw_PCA(), vec.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * vec: input vector(s); must have the same dimensionality and the
    /// same layout as the input data used at PCA phase, that is, if
    /// DATA_AS_ROW are specified, then `vec.cols==data.cols`
    /// (vector dimensionality) and `vec.rows` is the number of vectors to
    /// project, and the same is true for the PCA::DATA_AS_COL case.
    /// * result: output vectors; in case of PCA::DATA_AS_COL, the
    /// output matrix has as many columns as the number of input vectors, this
    /// means that `result.cols==vec.cols` and the number of rows match the
    /// number of principal components (for example, `maxComponents` parameter
    /// passed to the constructor).
    pub fn project_to(&self, vec: &core::Mat, result: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_PCA_project_const_Mat_Mat(self.as_raw_PCA(), vec.as_raw_Mat(), result.as_raw_Mat()) }.into_result()
    }
    
    /// Reconstructs vectors from their PC projections.
    /// 
    /// The methods are inverse operations to PCA::project. They take PC
    /// coordinates of projected vectors and reconstruct the original vectors.
    /// Unless all the principal components have been retained, the
    /// reconstructed vectors are different from the originals. But typically,
    /// the difference is small if the number of components is large enough (but
    /// still much smaller than the original vector dimensionality). As a
    /// result, PCA is used.
    /// ## Parameters
    /// * vec: coordinates of the vectors in the principal component
    /// subspace, the layout and size are the same as of PCA::project output
    /// vectors.
    pub fn back_project(&self, vec: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_PCA_backProject_const_Mat(self.as_raw_PCA(), vec.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// ## Parameters
    /// * vec: coordinates of the vectors in the principal component
    /// subspace, the layout and size are the same as of PCA::project output
    /// vectors.
    /// * result: reconstructed vectors; the layout and size are the same as
    /// of PCA::project input vectors.
    pub fn back_project_to(&self, vec: &core::Mat, result: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_PCA_backProject_const_Mat_Mat(self.as_raw_PCA(), vec.as_raw_Mat(), result.as_raw_Mat()) }.into_result()
    }
    
}

// Generating impl for trait cv::ParallelLoopBody (trait)
/// Base class for parallel data processors
pub trait ParallelLoopBody {
    #[doc(hidden)] fn as_raw_ParallelLoopBody(&self) -> *mut c_void;
}

impl<'a> ParallelLoopBody + 'a {

}

// boxed class cv::ParallelLoopBodyLambdaWrapper
#[allow(dead_code)]
pub struct ParallelLoopBodyLambdaWrapper {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::ParallelLoopBodyLambdaWrapper {
    fn drop(&mut self) {
        unsafe { sys::cv_ParallelLoopBodyLambdaWrapper_delete(self.ptr) };
    }
}
impl core::ParallelLoopBodyLambdaWrapper {
    pub fn as_raw_ParallelLoopBodyLambdaWrapper(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        ParallelLoopBodyLambdaWrapper {
            ptr
        }
    }
}

impl core::ParallelLoopBody for ParallelLoopBodyLambdaWrapper {
    fn as_raw_ParallelLoopBody(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::Param
#[allow(dead_code)]
pub struct Param {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::Param {
    fn drop(&mut self) {
        unsafe { sys::cv_Param_delete(self.ptr) };
    }
}
impl core::Param {
    pub fn as_raw_Param(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Param {
            ptr
        }
    }
}

// boxed class cv::Range
/// Template class specifying a continuous subsequence (slice) of a sequence.
/// 
/// The class is used to specify a row or a column span in a matrix ( Mat ) and for many other purposes.
/// Range(a,b) is basically the same as a:b in Matlab or a..b in Python. As in Python, start is an
/// inclusive left boundary of the range and end is an exclusive right boundary of the range. Such a
/// half-opened interval is usually denoted as <span lang='latex'>[start,end)</span> .
/// 
/// The static method Range::all() returns a special variable that means "the whole sequence" or "the
/// whole range", just like " : " in Matlab or " ... " in Python. All the methods and functions in
/// OpenCV that take Range support this special Range::all() value. But, of course, in case of your own
/// custom processing, you will probably have to check and handle it explicitly:
/// ```ignore
/// void my_function(..., const Range& r, ....)
/// {
/// if(r == Range::all()) {
/// // process all the data
/// }
/// else {
/// // process [r.start, r.end)
/// }
/// }
/// ```
#[allow(dead_code)]
pub struct Range {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::Range {
    fn drop(&mut self) {
        unsafe { sys::cv_Range_delete(self.ptr) };
    }
}
impl core::Range {
    pub fn as_raw_Range(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Range {
            ptr
        }
    }
}

impl Range {

    pub fn default() -> Result<core::Range> {
        unsafe { sys::cv_Range_Range() }.into_result().map(|x| core::Range { ptr: x })
    }
    
    pub fn new(_start: i32, _end: i32) -> Result<core::Range> {
        unsafe { sys::cv_Range_Range_int_int(_start, _end) }.into_result().map(|x| core::Range { ptr: x })
    }
    
    pub fn size(&self) -> Result<i32> {
        unsafe { sys::cv_Range_size_const(self.as_raw_Range()) }.into_result()
    }
    
    pub fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_Range_empty_const(self.as_raw_Range()) }.into_result()
    }
    
    pub fn all() -> Result<core::Range> {
        unsafe { sys::cv_Range_all() }.into_result().map(|x| core::Range { ptr: x })
    }
    
    pub fn start(&self) -> Result<i32> {
        unsafe { sys::cv_Range_start_const(self.as_raw_Range()) }.into_result()
    }
    
    pub fn end(&self) -> Result<i32> {
        unsafe { sys::cv_Range_end_const(self.as_raw_Range()) }.into_result()
    }
    
}

// boxed class cv::RotatedRect
/// The class represents rotated (i.e. not up-right) rectangles on a plane.
/// 
/// Each rectangle is specified by the center point (mass center), length of each side (represented by
/// #Size2f structure) and the rotation angle in degrees.
/// 
/// The sample below demonstrates how to use RotatedRect:
/// @snippet snippets/core_various.cpp RotatedRect_demo
/// ![image](https://docs.opencv.org/3.4.6/rotatedrect.png)
/// 
/// ## See also
/// CamShift, fitEllipse, minAreaRect, CvBox2D
#[allow(dead_code)]
pub struct RotatedRect {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::RotatedRect {
    fn drop(&mut self) {
        unsafe { sys::cv_RotatedRect_delete(self.ptr) };
    }
}
impl core::RotatedRect {
    pub fn as_raw_RotatedRect(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        RotatedRect {
            ptr
        }
    }
}

impl RotatedRect {

    /// default constructor
    pub fn default() -> Result<core::RotatedRect> {
        unsafe { sys::cv_RotatedRect_RotatedRect() }.into_result().map(|x| core::RotatedRect { ptr: x })
    }
    
    /// full constructor
    /// ## Parameters
    /// * center: The rectangle mass center.
    /// * size: Width and height of the rectangle.
    /// * angle: The rotation angle in a clockwise direction. When the angle is 0, 90, 180, 270 etc.,
    /// the rectangle becomes an up-right rectangle.
    pub fn new(center: core::Point2f, size: core::Size2f, angle: f32) -> Result<core::RotatedRect> {
        unsafe { sys::cv_RotatedRect_RotatedRect_Point2f_Size2f_float(center, size, angle) }.into_result().map(|x| core::RotatedRect { ptr: x })
    }
    
    /// Any 3 end points of the RotatedRect. They must be given in order (either clockwise or
    /// anticlockwise).
    pub fn for_points(point1: core::Point2f, point2: core::Point2f, point3: core::Point2f) -> Result<core::RotatedRect> {
        unsafe { sys::cv_RotatedRect_RotatedRect_Point2f_Point2f_Point2f(point1, point2, point3) }.into_result().map(|x| core::RotatedRect { ptr: x })
    }
    
    /// returns 4 vertices of the rectangle
    /// ## Parameters
    /// * pts: The points array for storing rectangle vertices. The order is bottomLeft, topLeft, topRight, bottomRight.
    pub fn points(&self, pts: &mut core::Point2f) -> Result<()> {
        unsafe { sys::cv_RotatedRect_points_const_Point2f_X(self.as_raw_RotatedRect(), pts) }.into_result()
    }
    
    /// returns the minimal up-right integer rectangle containing the rotated rectangle
    pub fn bounding_rect(&self) -> Result<core::Rect> {
        unsafe { sys::cv_RotatedRect_boundingRect_const(self.as_raw_RotatedRect()) }.into_result()
    }
    
    /// returns the minimal (exact) floating point rectangle containing the rotated rectangle, not intended for use with images
    pub fn bounding_rect2f(&self) -> Result<core::Rect2f> {
        unsafe { sys::cv_RotatedRect_boundingRect2f_const(self.as_raw_RotatedRect()) }.into_result()
    }
    
    /// returns the rectangle mass center
    pub fn center(&self) -> Result<core::Point2f> {
        unsafe { sys::cv_RotatedRect_center_const(self.as_raw_RotatedRect()) }.into_result()
    }
    
    /// returns width and height of the rectangle
    pub fn size(&self) -> Result<core::Size2f> {
        unsafe { sys::cv_RotatedRect_size_const(self.as_raw_RotatedRect()) }.into_result()
    }
    
    /// returns the rotation angle. When the angle is 0, 90, 180, 270 etc., the rectangle becomes an up-right rectangle.
    pub fn angle(&self) -> Result<f32> {
        unsafe { sys::cv_RotatedRect_angle_const(self.as_raw_RotatedRect()) }.into_result()
    }
    
}

// boxed class cv::TermCriteria
/// The class defining termination criteria for iterative algorithms.
/// 
/// You can initialize it by default constructor and then override any parameters, or the structure may
/// be fully initialized using the advanced variant of the constructor.
#[allow(dead_code)]
pub struct TermCriteria {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::TermCriteria {
    fn drop(&mut self) {
        unsafe { sys::cv_TermCriteria_delete(self.ptr) };
    }
}
impl core::TermCriteria {
    pub fn as_raw_TermCriteria(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        TermCriteria {
            ptr
        }
    }
}

impl TermCriteria {

    /// default constructor
    pub fn default() -> Result<core::TermCriteria> {
        unsafe { sys::cv_TermCriteria_TermCriteria() }.into_result().map(|x| core::TermCriteria { ptr: x })
    }
    
    /// ## Parameters
    /// * type: The type of termination criteria, one of TermCriteria::Type
    /// * maxCount: The maximum number of iterations or elements to compute.
    /// * epsilon: The desired accuracy or change in parameters at which the iterative algorithm stops.
    pub fn new(_type: i32, max_count: i32, epsilon: f64) -> Result<core::TermCriteria> {
        unsafe { sys::cv_TermCriteria_TermCriteria_int_int_double(_type, max_count, epsilon) }.into_result().map(|x| core::TermCriteria { ptr: x })
    }
    
    pub fn is_valid(&self) -> Result<bool> {
        unsafe { sys::cv_TermCriteria_isValid_const(self.as_raw_TermCriteria()) }.into_result()
    }
    
    pub fn _type(&self) -> Result<i32> {
        unsafe { sys::cv_TermCriteria_type_const(self.as_raw_TermCriteria()) }.into_result()
    }
    
    /// < the type of termination criteria: COUNT, EPS or COUNT + EPS
    pub fn max_count(&self) -> Result<i32> {
        unsafe { sys::cv_TermCriteria_maxCount_const(self.as_raw_TermCriteria()) }.into_result()
    }
    
    /// < the maximum number of iterations/elements
    pub fn epsilon(&self) -> Result<f64> {
        unsafe { sys::cv_TermCriteria_epsilon_const(self.as_raw_TermCriteria()) }.into_result()
    }
    
}

// boxed class cv::TickMeter
/// a Class to measure passing time.
/// 
/// The class computes passing time by counting the number of ticks per second. That is, the following code computes the
/// execution time in seconds:
/// ```ignore
/// TickMeter tm;
/// tm.start();
/// // do something ...
/// tm.stop();
/// std::cout << tm.getTimeSec();
/// ```
/// 
/// 
/// It is also possible to compute the average time over multiple runs:
/// ```ignore
/// TickMeter tm;
/// for (int i = 0; i < 100; i++)
/// {
/// tm.start();
/// // do something ...
/// tm.stop();
/// }
/// double average_time = tm.getTimeSec() / tm.getCounter();
/// std::cout << "Average time in second per iteration is: " << average_time << std::endl;
/// ```
/// 
/// ## See also
/// getTickCount, getTickFrequency
#[allow(dead_code)]
pub struct TickMeter {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::TickMeter {
    fn drop(&mut self) {
        unsafe { sys::cv_TickMeter_delete(self.ptr) };
    }
}
impl core::TickMeter {
    pub fn as_raw_TickMeter(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        TickMeter {
            ptr
        }
    }
}

impl TickMeter {

    /// the default constructor
    pub fn new() -> Result<core::TickMeter> {
        unsafe { sys::cv_TickMeter_TickMeter() }.into_result().map(|x| core::TickMeter { ptr: x })
    }
    
    /// starts counting ticks.
    pub fn start(&mut self) -> Result<()> {
        unsafe { sys::cv_TickMeter_start(self.as_raw_TickMeter()) }.into_result()
    }
    
    /// stops counting ticks.
    pub fn stop(&mut self) -> Result<()> {
        unsafe { sys::cv_TickMeter_stop(self.as_raw_TickMeter()) }.into_result()
    }
    
    /// returns counted ticks.
    pub fn get_time_ticks(&self) -> Result<i64> {
        unsafe { sys::cv_TickMeter_getTimeTicks_const(self.as_raw_TickMeter()) }.into_result()
    }
    
    /// returns passed time in microseconds.
    pub fn get_time_micro(&self) -> Result<f64> {
        unsafe { sys::cv_TickMeter_getTimeMicro_const(self.as_raw_TickMeter()) }.into_result()
    }
    
    /// returns passed time in milliseconds.
    pub fn get_time_milli(&self) -> Result<f64> {
        unsafe { sys::cv_TickMeter_getTimeMilli_const(self.as_raw_TickMeter()) }.into_result()
    }
    
    /// returns passed time in seconds.
    pub fn get_time_sec(&self) -> Result<f64> {
        unsafe { sys::cv_TickMeter_getTimeSec_const(self.as_raw_TickMeter()) }.into_result()
    }
    
    /// returns internal counter value.
    pub fn get_counter(&self) -> Result<i64> {
        unsafe { sys::cv_TickMeter_getCounter_const(self.as_raw_TickMeter()) }.into_result()
    }
    
    /// resets internal values.
    pub fn reset(&mut self) -> Result<()> {
        unsafe { sys::cv_TickMeter_reset(self.as_raw_TickMeter()) }.into_result()
    }
    
}

// boxed class cv::UMat
/// @todo document
#[allow(dead_code)]
pub struct UMat {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::UMat {
    fn drop(&mut self) {
        unsafe { sys::cv_UMat_delete(self.ptr) };
    }
}
impl core::UMat {
    pub fn as_raw_UMat(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        UMat {
            ptr
        }
    }
}

impl UMat {

    pub fn get_mat(&self, flags: i32) -> Result<core::Mat> {
        unsafe { sys::cv_UMat_getMat_const_int(self.as_raw_UMat(), flags) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    /// copies the matrix content to "m".
    pub fn copy_to(&self, m: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_UMat_copyTo_const_Mat(self.as_raw_UMat(), m.as_raw_Mat()) }.into_result()
    }
    
    /// copies those matrix elements to "m" that are marked with non-zero mask elements.
    pub fn copy_to_masked(&self, m: &mut core::Mat, mask: &core::Mat) -> Result<()> {
        unsafe { sys::cv_UMat_copyTo_const_Mat_Mat(self.as_raw_UMat(), m.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
    }
    
    /// converts matrix to another datatype with optional scaling. See cvConvertScale.
    ///
    /// ## C++ default parameters
    /// * alpha: 1
    /// * beta: 0
    pub fn convert_to(&self, m: &mut core::Mat, rtype: i32, alpha: f64, beta: f64) -> Result<()> {
        unsafe { sys::cv_UMat_convertTo_const_Mat_int_double_double(self.as_raw_UMat(), m.as_raw_Mat(), rtype, alpha, beta) }.into_result()
    }
    
    /// computes dot-product
    pub fn dot(&self, m: &core::Mat) -> Result<f64> {
        unsafe { sys::cv_UMat_dot_const_Mat(self.as_raw_UMat(), m.as_raw_Mat()) }.into_result()
    }
    
    /// increases the reference counter; use with care to avoid memleaks
    pub fn addref(&mut self) -> Result<()> {
        unsafe { sys::cv_UMat_addref(self.as_raw_UMat()) }.into_result()
    }
    
    /// decreases reference counter;
    pub fn release(&mut self) -> Result<()> {
        unsafe { sys::cv_UMat_release(self.as_raw_UMat()) }.into_result()
    }
    
    /// deallocates the matrix data
    pub fn deallocate(&mut self) -> Result<()> {
        unsafe { sys::cv_UMat_deallocate(self.as_raw_UMat()) }.into_result()
    }
    
    /// locates matrix header within a parent matrix. See below
    pub fn locate_roi(&self, whole_size: &mut core::Size, ofs: &mut core::Point) -> Result<()> {
        unsafe { sys::cv_UMat_locateROI_const_Size_Point(self.as_raw_UMat(), whole_size, ofs) }.into_result()
    }
    
    /// returns true iff the matrix data is continuous
    pub fn is_continuous(&self) -> Result<bool> {
        unsafe { sys::cv_UMat_isContinuous_const(self.as_raw_UMat()) }.into_result()
    }
    
    /// returns true if the matrix is a submatrix of another matrix
    pub fn is_submatrix(&self) -> Result<bool> {
        unsafe { sys::cv_UMat_isSubmatrix_const(self.as_raw_UMat()) }.into_result()
    }
    
    /// returns element size in bytes,
    pub fn elem_size(&self) -> Result<size_t> {
        unsafe { sys::cv_UMat_elemSize_const(self.as_raw_UMat()) }.into_result()
    }
    
    /// returns the size of element channel in bytes.
    pub fn elem_size1(&self) -> Result<size_t> {
        unsafe { sys::cv_UMat_elemSize1_const(self.as_raw_UMat()) }.into_result()
    }
    
    /// returns element type, similar to CV_MAT_TYPE(cvmat->type)
    pub fn typ(&self) -> Result<i32> {
        unsafe { sys::cv_UMat_type_const(self.as_raw_UMat()) }.into_result()
    }
    
    /// returns element type, similar to CV_MAT_DEPTH(cvmat->type)
    pub fn depth(&self) -> Result<i32> {
        unsafe { sys::cv_UMat_depth_const(self.as_raw_UMat()) }.into_result()
    }
    
    /// returns element type, similar to CV_MAT_CN(cvmat->type)
    pub fn channels(&self) -> Result<i32> {
        unsafe { sys::cv_UMat_channels_const(self.as_raw_UMat()) }.into_result()
    }
    
    /// returns step/elemSize1()
    ///
    /// ## C++ default parameters
    /// * i: 0
    pub fn step1(&self, i: i32) -> Result<size_t> {
        unsafe { sys::cv_UMat_step1_const_int(self.as_raw_UMat(), i) }.into_result()
    }
    
    /// returns true if matrix data is NULL
    pub fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_UMat_empty_const(self.as_raw_UMat()) }.into_result()
    }
    
    /// returns the total number of matrix elements
    pub fn total(&self) -> Result<size_t> {
        unsafe { sys::cv_UMat_total_const(self.as_raw_UMat()) }.into_result()
    }
    
    /// returns N if the matrix is 1-channel (N x ptdim) or ptdim-channel (1 x N) or (N x 1); negative number otherwise
    ///
    /// ## C++ default parameters
    /// * depth: -1
    /// * require_continuous: true
    pub fn check_vector(&self, elem_channels: i32, depth: i32, require_continuous: bool) -> Result<i32> {
        unsafe { sys::cv_UMat_checkVector_const_int_int_bool(self.as_raw_UMat(), elem_channels, depth, require_continuous) }.into_result()
    }
    
    pub fn handle(&self, access_flags: i32) -> Result<&mut c_void> {
        unsafe { sys::cv_UMat_handle_const_int(self.as_raw_UMat(), access_flags) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))
    }
    
    pub fn ndoffset(&self, ofs: &mut size_t) -> Result<()> {
        unsafe { sys::cv_UMat_ndoffset_const_size_t_X(self.as_raw_UMat(), ofs) }.into_result()
    }
    
    /// internal use method: updates the continuity flag
    pub fn update_continuity_flag(&mut self) -> Result<()> {
        unsafe { sys::cv_UMat_updateContinuityFlag(self.as_raw_UMat()) }.into_result()
    }
    
}

// boxed class cv::UMatData
/// Comma-separated Matrix Initializer
/// 
/// The class instances are usually not created explicitly.
/// Instead, they are created on "matrix << firstValue" operator.
/// 
/// The sample below initializes 2x2 rotation matrix:
/// 
/// \code
/// double angle = 30, a = cos(angle*CV_PI/180), b = sin(angle*CV_PI/180);
/// Mat R = (Mat_<double>(2,2) << a, -b, b, a);
/// \endcodethe constructor, created by "matrix << firstValue" operator, where matrix is cv::Mat
/// the operator that takes the next value and put it to the matrix
/// another form of conversion operator
#[allow(dead_code)]
pub struct UMatData {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::UMatData {
    fn drop(&mut self) {
        unsafe { sys::cv_UMatData_delete(self.ptr) };
    }
}
impl core::UMatData {
    pub fn as_raw_UMatData(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        UMatData {
            ptr
        }
    }
}

impl UMatData {

    pub fn lock(&mut self) -> Result<()> {
        unsafe { sys::cv_UMatData_lock(self.as_raw_UMatData()) }.into_result()
    }
    
    pub fn unlock(&mut self) -> Result<()> {
        unsafe { sys::cv_UMatData_unlock(self.as_raw_UMatData()) }.into_result()
    }
    
    pub fn host_copy_obsolete(&self) -> Result<bool> {
        unsafe { sys::cv_UMatData_hostCopyObsolete_const(self.as_raw_UMatData()) }.into_result()
    }
    
    pub fn device_copy_obsolete(&self) -> Result<bool> {
        unsafe { sys::cv_UMatData_deviceCopyObsolete_const(self.as_raw_UMatData()) }.into_result()
    }
    
    pub fn device_mem_mapped(&self) -> Result<bool> {
        unsafe { sys::cv_UMatData_deviceMemMapped_const(self.as_raw_UMatData()) }.into_result()
    }
    
    pub fn copy_on_map(&self) -> Result<bool> {
        unsafe { sys::cv_UMatData_copyOnMap_const(self.as_raw_UMatData()) }.into_result()
    }
    
    pub fn temp_u_mat(&self) -> Result<bool> {
        unsafe { sys::cv_UMatData_tempUMat_const(self.as_raw_UMatData()) }.into_result()
    }
    
    pub fn temp_copied_u_mat(&self) -> Result<bool> {
        unsafe { sys::cv_UMatData_tempCopiedUMat_const(self.as_raw_UMatData()) }.into_result()
    }
    
    pub fn mark_host_copy_obsolete(&mut self, flag: bool) -> Result<()> {
        unsafe { sys::cv_UMatData_markHostCopyObsolete_bool(self.as_raw_UMatData(), flag) }.into_result()
    }
    
    pub fn mark_device_copy_obsolete(&mut self, flag: bool) -> Result<()> {
        unsafe { sys::cv_UMatData_markDeviceCopyObsolete_bool(self.as_raw_UMatData(), flag) }.into_result()
    }
    
    pub fn mark_device_mem_mapped(&mut self, flag: bool) -> Result<()> {
        unsafe { sys::cv_UMatData_markDeviceMemMapped_bool(self.as_raw_UMatData(), flag) }.into_result()
    }
    
}

// boxed class cv::detail::CheckContext
#[allow(dead_code)]
pub struct CheckContext {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::CheckContext {
    fn drop(&mut self) {
        unsafe { sys::cv_CheckContext_delete(self.ptr) };
    }
}
impl core::CheckContext {
    pub fn as_raw_CheckContext(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        CheckContext {
            ptr
        }
    }
}

// boxed class cv::instr::NodeData
#[allow(dead_code)]
pub struct NodeData {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::NodeData {
    fn drop(&mut self) {
        unsafe { sys::cv_NodeData_delete(self.ptr) };
    }
}
impl core::NodeData {
    pub fn as_raw_NodeData(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        NodeData {
            ptr
        }
    }
}

impl NodeData {

    pub fn new(_ref: &mut core::NodeData) -> Result<core::NodeData> {
        unsafe { sys::cv_instr_NodeData_NodeData_NodeData(_ref.as_raw_NodeData()) }.into_result().map(|x| core::NodeData { ptr: x })
    }
    
    pub fn get_total_ms(&self) -> Result<f64> {
        unsafe { sys::cv_instr_NodeData_getTotalMs_const(self.as_raw_NodeData()) }.into_result()
    }
    
    pub fn get_mean_ms(&self) -> Result<f64> {
        unsafe { sys::cv_instr_NodeData_getMeanMs_const(self.as_raw_NodeData()) }.into_result()
    }
    
}

// boxed class cv::instr::NodeDataTls
#[allow(dead_code)]
pub struct NodeDataTls {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for core::NodeDataTls {
    fn drop(&mut self) {
        unsafe { sys::cv_NodeDataTls_delete(self.ptr) };
    }
}
impl core::NodeDataTls {
    pub fn as_raw_NodeDataTls(&self) -> *mut c_void { self.ptr }
    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        NodeDataTls {
            ptr
        }
    }
}

impl NodeDataTls {

    pub fn new() -> Result<core::NodeDataTls> {
        unsafe { sys::cv_instr_NodeDataTls_NodeDataTls() }.into_result().map(|x| core::NodeDataTls { ptr: x })
    }
    
}

pub const CV_16SC1: i32 = 0x3; // 3
pub const CV_16SC2: i32 = 0xb; // 11
pub const CV_16SC3: i32 = 0x13; // 19
pub const CV_16SC4: i32 = 0x1b; // 27
pub const CV_16UC1: i32 = 0x2; // 2
pub const CV_16UC2: i32 = 0xa; // 10
pub const CV_16UC3: i32 = 0x12; // 18
pub const CV_16UC4: i32 = 0x1a; // 26
pub const CV_32FC1: i32 = 0x5; // 5
pub const CV_32FC2: i32 = 0xd; // 13
pub const CV_32FC3: i32 = 0x15; // 21
pub const CV_32FC4: i32 = 0x1d; // 29
pub const CV_32SC1: i32 = 0x4; // 4
pub const CV_32SC2: i32 = 0xc; // 12
pub const CV_32SC3: i32 = 0x14; // 20
pub const CV_32SC4: i32 = 0x1c; // 28
pub const CV_64FC1: i32 = 0x6; // 6
pub const CV_64FC2: i32 = 0xe; // 14
pub const CV_64FC3: i32 = 0x16; // 22
pub const CV_64FC4: i32 = 0x1e; // 30
pub const CV_8SC1: i32 = 0x1; // 1
pub const CV_8SC2: i32 = 0x9; // 9
pub const CV_8SC3: i32 = 0x11; // 17
pub const CV_8SC4: i32 = 0x19; // 25
pub const CV_8UC1: i32 = 0x0; // 0
pub const CV_8UC2: i32 = 0x8; // 8
pub const CV_8UC3: i32 = 0x10; // 16
pub const CV_8UC4: i32 = 0x18; // 24
pub const CV_DEPTH_MAX: i32 = 0x8; // 8
pub const CV_MAT_CN_MASK: i32 = 0xff8; // 4088
pub const CV_MAT_CONT_FLAG: i32 = 0x4000; // 16384
pub const CV_MAT_DEPTH_MASK: i32 = 0x7; // 7
pub const CV_MAT_TYPE_MASK: i32 = 0xfff; // 4095
pub const CV_SUBMAT_FLAG: i32 = 0x8000; // 32768
pub static CV_VERSION: &'static str = "3.4.6";
pub const ENUM_LOG_LEVEL_FORCE_INT: i32 = 0x7fffffff; // 2147483647
pub const Mat_CONTINUOUS_FLAG: i32 = 0x4000; // 16384
pub const Mat_SUBMATRIX_FLAG: i32 = 0x8000; // 32768
pub const _InputArray_CUDA_GPU_MAT: i32 = 0x90000; // 589824
pub const _InputArray_CUDA_HOST_MEM: i32 = 0x80000; // 524288
pub const _InputArray_EXPR: i32 = 0x60000; // 393216
pub const _InputArray_FIXED_SIZE: i32 = 0x40000000; // 1073741824
pub const _InputArray_FIXED_TYPE: i32 = 0x80000000; // -2147483648
pub const _InputArray_KIND_MASK: i32 = 0x1f0000; // 2031616
pub const _InputArray_MATX: i32 = 0x20000; // 131072
pub const _InputArray_NONE: i32 = 0x0; // 0
pub const _InputArray_OPENGL_BUFFER: i32 = 0x70000; // 458752
pub const _InputArray_STD_ARRAY: i32 = 0xe0000; // 917504
pub const _InputArray_STD_ARRAY_MAT: i32 = 0xf0000; // 983040
pub const _InputArray_STD_BOOL_VECTOR: i32 = 0xc0000; // 786432
pub const _InputArray_STD_VECTOR: i32 = 0x30000; // 196608
pub const _InputArray_STD_VECTOR_CUDA_GPU_MAT: i32 = 0xd0000; // 851968
pub const _InputArray_STD_VECTOR_MAT: i32 = 0x50000; // 327680
pub const _InputArray_STD_VECTOR_UMAT: i32 = 0xb0000; // 720896
pub const _InputArray_STD_VECTOR_VECTOR: i32 = 0x40000; // 262144
pub const _InputArray_UMAT: i32 = 0xa0000; // 655360
pub const _OutputArray_DEPTH_MASK_16S: i32 = 0x8; // 8
pub const _OutputArray_DEPTH_MASK_16U: i32 = 0x4; // 4
pub const _OutputArray_DEPTH_MASK_32F: i32 = 0x20; // 32
pub const _OutputArray_DEPTH_MASK_32S: i32 = 0x10; // 16
pub const _OutputArray_DEPTH_MASK_64F: i32 = 0x40; // 64
pub const _OutputArray_DEPTH_MASK_8S: i32 = 0x2; // 2
pub const _OutputArray_DEPTH_MASK_8U: i32 = 0x1; // 1
pub const _OutputArray_DEPTH_MASK_ALL: i32 = 0x7f; // 127
pub const _OutputArray_DEPTH_MASK_ALL_BUT_8S: i32 = 0x7d; // 125
pub const _OutputArray_DEPTH_MASK_FLT: i32 = 0x60; // 96
pub use crate::hub::manual::core::*;
