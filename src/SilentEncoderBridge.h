#pragma once
#include <libOTe/Tools/ExConvCode/ExConvCode.h>
#include <libOTe/Tools/CoeffCtx.h>
#include <cryptoTools/Common/Defines.h>
#include <rust/cxx.h>

namespace osuCryptoBridge
{

    struct ExConvCodeBridge : public oc::ExConvCode
    {
        void config(
            oc::u64 messageSize,
            oc::u64 codeSize,
            oc::u64 expanderWeight,
            oc::u64 accumulatorSize)
        {
            ExConvCode::config(messageSize, codeSize, expanderWeight, accumulatorSize);
        }

        void dualEncodeByte(rust::Slice<oc::u8> e)
        {
            dualEncode<oc::u8, oc::CoeffCtxGF2>(e.begin(), {});
        }

        void dualEncodeBlock(rust::Slice<oc::block> e)
        {
            dualEncode<oc::block, oc::CoeffCtxGF2>(e.begin(), {});
        }

        void dualEncode2Block(rust::Slice<oc::block> e0, rust::Slice<oc::u8> e1)
        {
            dualEncode2<oc::block, oc::u8, oc::CoeffCtxGF2>(e0.begin(), e1.begin(), {});
        }
    };

    std::unique_ptr<ExConvCodeBridge> newExConvCode()
    {
        return std::make_unique<ExConvCodeBridge>();
    }
}
