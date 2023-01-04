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
	passthroughModule := &harness.Module{
		Name:      "passthrough",
		Path:      "tests/modules/passthrough/passthrough.go",
		Signature: "github.com/loopholelabs/scale-signature-http",
	}

	nextModule := &harness.Module{
		Name:      "next",
		Path:      "tests/modules/next/next.go",
		Signature: "github.com/loopholelabs/scale-signature-http",
	}

	fileModule := &harness.Module{
		Name:      "file",
		Path:      "tests/modules/file/file.go",
		Signature: "github.com/loopholelabs/scale-signature-http",
	}

	networkModule := &harness.Module{
		Name:      "network",
		Path:      "tests/modules/network/network.go",
		Signature: "github.com/loopholelabs/scale-signature-http",
	}

	panicModule := &harness.Module{
		Name:      "panic",
		Path:      "tests/modules/panic/panic.go",
		Signature: "github.com/loopholelabs/scale-signature-http",
	}

	modules := []*harness.Module{passthroughModule, nextModule, fileModule, networkModule, panicModule}

	generatedModules := harness.Setup(t, modules, "github.com/loopholelabs/scale-signature-http/tests/modules")

	var testCases = []TestCase{
		{
			Name:   "Passthrough",
			Module: passthroughModule,
			Run: func(scaleFunc *scalefunc.ScaleFunc, t *testing.T) {
				r, err := runtime.New(context.Background(), New, []*scalefunc.ScaleFunc{scaleFunc})
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

				r, err := runtime.New(context.Background(), New, []*scalefunc.ScaleFunc{scaleFunc})
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

				r, err := runtime.New(context.Background(), New, []*scalefunc.ScaleFunc{scaleFunc})
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
				r, err := runtime.New(context.Background(), New, []*scalefunc.ScaleFunc{scaleFunc})
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
				r, err := runtime.New(context.Background(), New, []*scalefunc.ScaleFunc{scaleFunc})
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
				r, err := runtime.New(context.Background(), New, []*scalefunc.ScaleFunc{scaleFunc})
				require.NoError(t, err)

				i, err := r.Instance(nil)
				require.NoError(t, err)

				err = i.Run(context.Background())
				require.Error(t, err)
			},
		},
	}

	for _, testCase := range testCases {
		t.Run(testCase.Name, func(t *testing.T) {

			module, err := os.ReadFile(generatedModules[testCase.Module])
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
}
