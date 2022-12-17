/*
	Copyright 2022 Loophole Labs

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		   http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.
*/

package http

import (
	"context"
	"errors"
	runtime "github.com/loopholelabs/scale/go"
	"github.com/loopholelabs/scale/go/tests/harness"
	"github.com/loopholelabs/scalefile/scalefunc"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"os"
	"testing"
)

type TestCase struct {
	Name   string
	Module *harness.Module
	Run    func(*scalefunc.ScaleFunc, *testing.T)
}

func TestSignature(t *testing.T) {
	passthroughModuleGo := &harness.Module{
		Name:      "passthrough",
		Path:      "tests/modules/go/passthrough/passthrough.go",
		Signature: "github.com/loopholelabs/scale-signature-http",
	}

	nextModuleGo := &harness.Module{
		Name:      "next",
		Path:      "tests/modules/go/next/next.go",
		Signature: "github.com/loopholelabs/scale-signature-http",
	}

	fileModuleGo := &harness.Module{
		Name:      "file",
		Path:      "tests/modules/go/file/file.go",
		Signature: "github.com/loopholelabs/scale-signature-http",
	}

	networkModuleGo := &harness.Module{
		Name:      "network",
		Path:      "tests/modules/go/network/network.go",
		Signature: "github.com/loopholelabs/scale-signature-http",
	}

	panicModuleGo := &harness.Module{
		Name:      "panic",
		Path:      "tests/modules/go/panic/panic.go",
		Signature: "github.com/loopholelabs/scale-signature-http",
	}

	goModules := []*harness.Module{passthroughModule, nextModule, fileModule, networkModule, panicModule}

	generatedGoModules := harness.GoSetup(t, goModules, "github.com/loopholelabs/scale-signature-http/tests/modules/go")

	var goTestCases = []TestCase{
		{
			Name:   "Passthrough",
			Module: passthroughModule,
			Run: func(scaleFunc *scalefunc.ScaleFunc, t *testing.T) {
				r, err := runtime.New(context.Background(), New(), []*scalefunc.ScaleFunc{scaleFunc})
				require.NoError(t, err)

				i, err := r.Instance(nil)
				require.NoError(t, err)

				i.Context().Response().SetBody("Test Data")

				err = i.Run(context.Background())
				assert.NoError(t, err)

				assert.Equal(t, []byte("Test Data"), i.Context().Response().Body())
			},
		},
		{
			Name:   "Next",
			Module: nextModule,
			Run: func(scaleFunc *scalefunc.ScaleFunc, t *testing.T) {
				next := func(ctx *Context) (*Context, error) {
					ctx.Response().SetBody("Hello, World!")
					return ctx, nil
				}

				r, err := runtime.New(context.Background(), New(), []*scalefunc.ScaleFunc{scaleFunc})
				require.NoError(t, err)

				i, err := r.Instance(next)
				require.NoError(t, err)

				i.Context().Response().SetBody("Test Data")

				err = i.Run(context.Background())
				assert.NoError(t, err)

				assert.Equal(t, []byte("Hello, World!"), i.Context().Response().Body())
			},
		},
		{
			Name:   "NextError",
			Module: nextModule,
			Run: func(scaleFunc *scalefunc.ScaleFunc, t *testing.T) {
				next := func(ctx *Context) (*Context, error) {
					return nil, errors.New("next error")
				}

				r, err := runtime.New(context.Background(), New(), []*scalefunc.ScaleFunc{scaleFunc})
				require.NoError(t, err)

				i, err := r.Instance(next)
				require.NoError(t, err)

				err = i.Run(context.Background())
				require.ErrorIs(t, err, errors.New("next error"))
			},
		},
		{
			Name:   "File",
			Module: fileModule,
			Run: func(scaleFunc *scalefunc.ScaleFunc, t *testing.T) {
				r, err := runtime.New(context.Background(), New(), []*scalefunc.ScaleFunc{scaleFunc})
				require.NoError(t, err)

				i, err := r.Instance(nil)
				require.NoError(t, err)

				err = i.Run(context.Background())
				require.Error(t, err)
			},
		},
		{
			Name:   "Network",
			Module: networkModule,
			Run: func(scaleFunc *scalefunc.ScaleFunc, t *testing.T) {
				r, err := runtime.New(context.Background(), New(), []*scalefunc.ScaleFunc{scaleFunc})
				require.NoError(t, err)

				i, err := r.Instance(nil)
				require.NoError(t, err)

				err = i.Run(context.Background())
				require.Error(t, err)
			},
		},
		{
			Name:   "Panic",
			Module: panicModule,
			Run: func(scaleFunc *scalefunc.ScaleFunc, t *testing.T) {
				r, err := runtime.New(context.Background(), New(), []*scalefunc.ScaleFunc{scaleFunc})
				require.NoError(t, err)

				i, err := r.Instance(nil)
				require.NoError(t, err)

				err = i.Run(context.Background())
				require.Error(t, err)
			},
		},
	}

	passthroughModuleRust := &harness.Module{
		Name:      "passthrough",
		Path:      "tests/modules/rust/passthrough.rs",
		Signature: "scale_signature_http",
	}

	nextModuleRust := &harness.Module{
		Name:      "next",
		Path:      "tests/modules/rust/next.rs",
		Signature: "scale_signature_http",
	}

	fileModuleRust := &harness.Module{
		Name:      "file",
		Path:      "tests/modules/rust/file.rs",
		Signature: "scale_signature_http",
	}

	networkModuleRust := &harness.Module{
		Name:      "network",
		Path:      "tests/modules/rust/network.rs",
		Signature: "scale_signature_http",
	}

	panicModuleRust := &harness.Module{
		Name:      "panic",
		Path:      "tests/modules/rust/panic.rs",
		Signature: "scale_signature_http",
	}

	rustModules := []*harness.Module{passthroughModuleRust, nextModuleRust, fileModuleRust, networkModuleRust, panicModuleRust}

	generatedRustModules := harness.RustSetup(t, rustModules, "github.com/loopholelabs/scale-signature-http/tests/modules/rust")

	var rustTestCases = []TestCase{
		{
			Name:   "Passthrough",
			Module: passthroughModule,
			Run: func(scaleFunc *scalefunc.ScaleFunc, t *testing.T) {
				r, err := runtime.New(context.Background(), New(), []*scalefunc.ScaleFunc{scaleFunc})
				require.NoError(t, err)

				i, err := r.Instance(nil)
				require.NoError(t, err)

				i.Context().Response().SetBody("Test Data")

				err = i.Run(context.Background())
				assert.NoError(t, err)

				assert.Equal(t, []byte("Test Data"), i.Context().Response().Body())
			},
		},
		{
			Name:   "Next",
			Module: nextModule,
			Run: func(scaleFunc *scalefunc.ScaleFunc, t *testing.T) {
				next := func(ctx *Context) (*Context, error) {
					ctx.Response().SetBody("Hello, World!")
					return ctx, nil
				}

				r, err := runtime.New(context.Background(), New(), []*scalefunc.ScaleFunc{scaleFunc})
				require.NoError(t, err)

				i, err := r.Instance(next)
				require.NoError(t, err)

				i.Context().Response().SetBody("Test Data")

				err = i.Run(context.Background())
				assert.NoError(t, err)

				assert.Equal(t, []byte("Hello, World!"), i.Context().Response().Body())
			},
		},
		{
			Name:   "NextError",
			Module: nextModule,
			Run: func(scaleFunc *scalefunc.ScaleFunc, t *testing.T) {
				next := func(ctx *Context) (*Context, error) {
					return nil, errors.New("next error")
				}

				r, err := runtime.New(context.Background(), New(), []*scalefunc.ScaleFunc{scaleFunc})
				require.NoError(t, err)

				i, err := r.Instance(next)
				require.NoError(t, err)

				err = i.Run(context.Background())
				require.ErrorIs(t, err, errors.New("next error"))
			},
		},
		{
			Name:   "File",
			Module: fileModule,
			Run: func(scaleFunc *scalefunc.ScaleFunc, t *testing.T) {
				r, err := runtime.New(context.Background(), New(), []*scalefunc.ScaleFunc{scaleFunc})
				require.NoError(t, err)

				i, err := r.Instance(nil)
				require.NoError(t, err)

				err = i.Run(context.Background())
				require.Error(t, err)
			},
		},
		{
			Name:   "Network",
			Module: networkModule,
			Run: func(scaleFunc *scalefunc.ScaleFunc, t *testing.T) {
				r, err := runtime.New(context.Background(), New(), []*scalefunc.ScaleFunc{scaleFunc})
				require.NoError(t, err)

				i, err := r.Instance(nil)
				require.NoError(t, err)

				err = i.Run(context.Background())
				require.Error(t, err)
			},
		},
		{
			Name:   "Panic",
			Module: panicModule,
			Run: func(scaleFunc *scalefunc.ScaleFunc, t *testing.T) {
				r, err := runtime.New(context.Background(), New(), []*scalefunc.ScaleFunc{scaleFunc})
				require.NoError(t, err)

				i, err := r.Instance(nil)
				require.NoError(t, err)

				err = i.Run(context.Background())
				require.Error(t, err)
			},
		},
	}
	for _, testCase := range goTestCases {
		t.Run(testCase.Name, func(t *testing.T) {

			module, err := os.ReadFile(generatedGoModules[testCase.Module])
			require.NoError(t, err)

			scaleFunc := &scalefunc.ScaleFunc{
				Version:   "TestVersion",
				Name:      "TestName",
				Signature: "http@v0.1.1",
				Language:  "go",
				Function:  module,
			}
			testCase.Run(scaleFunc, t)
		})
	}

	for _, testCase := range rustTestCases {
		t.Run(testCase.Name, func(t *testing.T) {

			module, err := os.ReadFile(generatedRustModules[testCase.Module])
			require.NoError(t, err)

			scaleFunc := &scalefunc.ScaleFunc{
				Version:   "TestVersion",
				Name:      "TestName",
				Signature: "http@v0.1.1",
				Language:  "rust",
				Function:  module,
			}
			testCase.Run(scaleFunc, t)
		})
	}
}
