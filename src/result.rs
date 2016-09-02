pub type HRESULT = ::w::HRESULT; // re-export HRESULT from WinAPI

#[derive(Debug)]
pub enum Error {
    OperationAborted,
    AccessDenied, // UnauthorizedAccessException in .NET (https://msdn.microsoft.com/en-us/library/awy7adbx(v=vs.110).aspx)
    UnspecifiedFailure,
    InvalidHandle,
    InvalidArgument,
    NoSuchInterface,
    NotImplemented, // NotImplementedException in .NET (https://msdn.microsoft.com/en-us/library/9ztbc5s1(v=vs.110).aspx)
    OutOfMemory, // OutOfMemoryException in .NET (https://msdn.microsoft.com/en-us/library/9ztbc5s1(v=vs.110).aspx)
    InvalidPointer,
    UnexpectedFailure,
    Other(HRESULT)
}

impl Error {
    pub fn from_hresult(hr: HRESULT) -> Error {
        use Error::*;

        match hr {
            ::w::E_ABORT => OperationAborted,
            ::w::E_ACCESSDENIED => AccessDenied,
            ::w::E_FAIL => UnspecifiedFailure,
            ::w::E_HANDLE => InvalidHandle,
            ::w::E_INVALIDARG => InvalidArgument,
            ::w::E_NOINTERFACE => NoSuchInterface,
            ::w::E_NOTIMPL => NotImplemented,
            ::w::E_OUTOFMEMORY => OutOfMemory,
            ::w::E_POINTER => InvalidPointer,
            ::w::E_UNEXPECTED => UnexpectedFailure,
            _ => Other(hr)
        }
    }

    pub fn as_hresult(&self) -> HRESULT {
        use Error::*;

        match *self { 
            OperationAborted => ::w::E_ABORT,
            AccessDenied => ::w::E_ACCESSDENIED,
            UnspecifiedFailure => ::w::E_FAIL,
            InvalidHandle => ::w::E_HANDLE,
            InvalidArgument => ::w::E_INVALIDARG,
            NoSuchInterface => ::w::E_NOINTERFACE,
            NotImplemented => ::w::E_NOTIMPL,
            OutOfMemory => ::w::E_OUTOFMEMORY,
            InvalidPointer => ::w::E_POINTER,
            UnexpectedFailure => ::w::E_UNEXPECTED,
            Other(hr) => hr,
        }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;