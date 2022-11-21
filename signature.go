// Code generated by scale-signature-go v0.0.1, DO NOT EDIT.
// source: signature/http/signature.proto

package http

import (
	"bytes"
	_ "embed"
	"github.com/loopholelabs/polyglot-go"
	"github.com/loopholelabs/scale-signature"
	"unsafe"
)

//go:embed signature.yaml
var embeddedSignatureFile []byte
var signatureFile *signature.Definition

func init() {
	var err error
	signatureFile, err = signature.Decode(bytes.NewReader(embeddedSignatureFile))
	if err != nil {
		panic(err)
	}
}

var _ signature.RuntimeContext = (*RuntimeContext)(nil)
var _ signature.GuestContext = (*GuestContext)(nil)
var _ signature.Signature = (*Context)(nil)
var _ signature.Context = (*Context)(nil)

var (
	writeBuffer = polyglot.NewBuffer()
	readBuffer  []byte
)

// Context is a context object for an incoming request. It is meant to be used
// inside the Scale function.
type Context struct {
	generated *HttpContext
	buffer    *polyglot.Buffer
}

type GuestContext Context
type RuntimeContext Context

// New creates a new empty Context
func New() *Context {
	return &Context{
		generated: NewHttpContext(),
		buffer:    polyglot.NewBuffer(),
	}
}

func (x *Context) GuestContext() signature.GuestContext {
	return (*GuestContext)(x)
}

func (x *Context) RuntimeContext() signature.RuntimeContext {
	return (*RuntimeContext)(x)
}

func (x *Context) Version() string {
	return signatureFile.Version
}

func (x *Context) Name() string {
	return signatureFile.Name
}

func (x *Context) Resize(size uint32) uint32 {
	return Resize(size)
}

// ToWriteBuffer serializes the Context into the global writeBuffer and returns the pointer to the buffer and its size
//
// This method should only be used to read the Context from the Scale Runtime.
// Users should not use this method.
func (x *GuestContext) ToWriteBuffer() (uint32, uint32) {
	writeBuffer.Reset()
	x.generated.internalEncode(writeBuffer)
	underlying := writeBuffer.Bytes()
	ptr := &underlying[0]
	unsafePtr := uintptr(unsafe.Pointer(ptr))
	return uint32(unsafePtr), uint32(writeBuffer.Len())
}

// FromReadBuffer deserializes the data into the Context from the global readBuffer
//
// It assumes that the readBuffer has been filled with the data from the Scale Runtime after
// a call to the Resize method
func (x *GuestContext) FromReadBuffer() error {
	return x.generated.internalDecode(readBuffer)
}

// Read reads the context from the given byte slice and returns an error if one occurred
//
// This method is meant to be used by the Scale Runtime to deserialize the Context
func (x *RuntimeContext) Read(b []byte) error {
	return x.generated.internalDecode(b)
}

// Write writes the context into a byte slice and returns it
func (x *RuntimeContext) Write() []byte {
	x.buffer.Reset()
	x.generated.internalEncode(x.buffer)
	return x.buffer.Bytes()
}

// Next calls the next host function after writing the Context into the global writeBuffer,
// then it reads the result from the global readBuffer back into the Context
func (x *Context) Next() *Context {
	next(x.GuestContext().ToWriteBuffer())
	_ = x.GuestContext().FromReadBuffer()
	return x
}

// Generated is not meant to be used directly. It is meant to be used by the Scale Runtime.
func (x *Context) Generated() *HttpContext {
	return x.generated
}
