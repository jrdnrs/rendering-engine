// https://theorangeduck.com/page/avoiding-shader-conditionals
float gt(float x, float y) {
  return max(sign(x - y), 0.0);
}

float lt(float x, float y) {
  return max(sign(y - x), 0.0);
}