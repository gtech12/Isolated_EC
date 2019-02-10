/*!
   ufire.co for links to documentation, examples, and libraries
   github.com/u-fire for feature requests, bug reports, and  questions
   questions@ufire.co to get in touch with someone

   This example is compatible with hardware version 1, firmware version 1.

   It demonstrates the minimal amount of code and setup to take a reading
   using temperature compensation.
 */
#include <uFire_EC.h>

uFire_EC ec;

void setup()
{
  Serial.begin(9600);

  // tell the device to use temperature compensation
  ec.useTemperatureCompensation(true);

  // the temperature to adjust the readings to
  ec.setTempConstant(25);
}

void loop()
{
  // Calling measureEC with no parameters will automatically take a temperature reading
  // if useTemperatureCompensation has been set to true.
  ec.measureEC();
  Serial.print("mS/cm: ");
  Serial.println(ec.mS);
  Serial.println("-----");
  delay(1000);
}
