#ifdef __has_include
#if __has_include("ArduinoJson.h")
#include "uFire_EC_MP.h"
#include <ArduinoJson.h>

void uFire_EC_MP::begin(uFire_EC *p_ec)
{
  ec = p_ec;
  ec->begin();
}

String uFire_EC_MP::processMP(String rx_string)
{
  String cmd = rx_string.substring(0, rx_string.indexOf(" ", 0));
  cmd.trim();
  rx_string.remove(0, rx_string.indexOf(" ", 0));
  rx_string.trim();
  String parameter = rx_string.substring(0, rx_string.indexOf(" ", 0));
  parameter.trim();

  String value = "";
  if (cmd == "ec")            value = ec_measure(parameter);
  if (cmd == "etc")           value = ec_temp_const(parameter);
  if (cmd == "eco")           value = ec_temp_coeff(parameter);
  if (cmd == "ehrf")          value = ec_high_ref(parameter);
  if (cmd == "ehr")           value = ec_high_read();
  if (cmd == "elrf")          value = ec_low_ref(parameter);
  if (cmd == "elr")           value = ec_low_read();
  if (cmd == "ecr")           value = ec_reset();
  if (cmd == "ecc")           value = ec_connected();
  if (cmd == "eo")            value = ec_offset(parameter);
  if (cmd == "ect")           value = ec_temperature();

  if (value != "")
  {
    this->value = value.toFloat();
    return value;
  } else
  {
    this->value = -1;
    return "";
  }
}

String uFire_EC_MP::ec_reset()
{
  String output;
  const size_t bufferSize = JSON_OBJECT_SIZE(1) + 20;
  DynamicJsonDocument doc(bufferSize);
  doc["ecr"] = "ecr";
  ec->reset();
  serializeMsgPack(doc, output);
  return output;
}

String uFire_EC_MP::ec_connected()
{
  String output;
  const size_t bufferSize = JSON_OBJECT_SIZE(1) + 20;
  DynamicJsonDocument doc(bufferSize);
  doc["ecc"] = ec->connected();
  serializeMsgPack(doc, output);
  return output;
}

String uFire_EC_MP::ec_temp_const(String parameter)
{
  String output;

  if (parameter.length())
  {
    ec->setTempConstant(parameter.toFloat());
  }

  const size_t bufferSize = JSON_OBJECT_SIZE(1) + 20;
  DynamicJsonDocument doc(bufferSize);
  doc["etc"] = ec->getTempConstant();
  serializeMsgPack(doc, output);
  return output;
}

String uFire_EC_MP::ec_temp_coeff(String parameter)
{
  String output;

  if (parameter.length())
  {
    ec->setTempCoefficient(parameter.toFloat());
  }

  const size_t bufferSize = JSON_OBJECT_SIZE(1) + 20;
  DynamicJsonDocument doc(bufferSize);
  doc["eco"] = ec->getTempCoefficient();
  serializeMsgPack(doc, output);
  return output;

}

String uFire_EC_MP::ec_high_ref(String parameter)
{
  String output;

  if (parameter.length())
  {
    ec->calibrateProbeHigh(parameter.toFloat(), ec->measureTemp());
  }

  const size_t bufferSize = JSON_OBJECT_SIZE(1) + 20;
  DynamicJsonDocument doc(bufferSize);
  float highReference = ec->getCalibrateHighReference();
  if (isnan(highReference)) {
    doc["ehrf"]  = "-";
  }
  else {
    doc["ehrf"] = highReference;
  }
  serializeMsgPack(doc, output);
  return output;
}

String uFire_EC_MP::ec_high_read()
{
  String output;

  const size_t bufferSize = JSON_OBJECT_SIZE(1) + 20;
  DynamicJsonDocument doc(bufferSize);
  float highRead = ec->getCalibrateHighReading();
  if (isnan(highRead)) {
    doc["ehr"]  = "-";
  }
  else {
    doc["ehr"] = highRead;
  }
  serializeMsgPack(doc, output);
  return output;
}

String uFire_EC_MP::ec_low_ref(String parameter)
{
  String output;

  if (parameter.length())
  {
    ec->calibrateProbeLow(parameter.toFloat(), ec->measureTemp());
  }

  const size_t bufferSize = JSON_OBJECT_SIZE(1) + 20;
  DynamicJsonDocument doc(bufferSize);
  float lowRef = ec->getCalibrateLowReference();
  if (isnan(lowRef)) {
    doc["elrf"]  = "-";
  }
  else {
    doc["elrf"] = lowRef;
  }
  serializeMsgPack(doc, output);
  return output;
}

String uFire_EC_MP::ec_low_read()
{
  String output;

  const size_t bufferSize = JSON_OBJECT_SIZE(1) + 20;
  DynamicJsonDocument doc(bufferSize);
  float lowRead = ec->getCalibrateLowReading();
  if (isnan(lowRead)) {
    doc["elr"]  = "-";
  }
  else {
    doc["elr"] = lowRead;
  }
  serializeMsgPack(doc, output);
  return output;
}

String uFire_EC_MP::ec_measure(String temperature)
{
  String output;

  const size_t bufferSize = JSON_OBJECT_SIZE(1) + 20;
  DynamicJsonDocument doc(bufferSize);
  doc["ec"] = floor(ec->measureEC(temperature.toFloat()) * 100.0 + 0.5) / 100.0;
  serializeMsgPack(doc, output);
  return output;
}

String uFire_EC_MP::ec_offset(String parameter)
{
  String output;

  if (parameter.length())
  {
    ec->calibrateProbe(parameter.toFloat(), ec->measureTemp());
  }

  const size_t bufferSize = JSON_OBJECT_SIZE(1) + 20;
  DynamicJsonDocument doc(bufferSize);
  float offset = ec->getCalibrateOffset();
  if (isnan(offset)) {
    doc["eo"]  = "-";
  }
  else {
    doc["eo"] = offset;
  }
  serializeMsgPack(doc, output);

  return output;
}

String uFire_EC_MP::ec_temperature()
{
  String output;
  const size_t bufferSize = JSON_OBJECT_SIZE(1) + 20;
  DynamicJsonDocument doc(bufferSize);
  doc["ect"] = floor(ec->measureTemp() * 100.0 + 0.5) / 100.0;
  serializeMsgPack(doc, output);
  return output;
}
#endif
#endif
