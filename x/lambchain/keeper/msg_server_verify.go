package keeper

import (
	"context"
	"fmt"
	"strings"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"lambchain/x/lambchain/types"
)

func (k msgServer) Verify(goCtx context.Context, msg *types.MsgVerify) (*types.MsgVerifyResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)
	_ = ctx

	if !verify(msg.Proof) {
		fmt.Println("Invalid proof: ", msg.Proof)
		return nil, types.ErrSample
	} else {
		fmt.Println("Valid proof: ", msg.Proof)
		return &types.MsgVerifyResponse{}, nil
	}
}

func verify(proof string) bool {
	return !strings.Contains(proof, "invalid")
}
