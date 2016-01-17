#pragma version(1)
#pragma rs java_package_name(com.github.krukon.tutoratamicamera)
#pragma rs_fp_relaxed

int imageWidth;
int imageHeight;

rs_allocation in;
rs_allocation out;
rs_script script;

void root(const uchar4 *v_in, uchar4 *v_out, const void *usrData, uint32_t x, uint32_t y) {
    float4 f4 = rsUnpackColor8888(*v_in);
    float val = (f4.r + f4.g + f4.b) / 3.0;
    f4.r = f4.g = f4.b = val;

    float3 mono = {f4.r, f4.g, f4.b};

    *v_out = rsPackColorTo8888(mono);
}

void filter() {
    rsForEach(script, in, out);
}
