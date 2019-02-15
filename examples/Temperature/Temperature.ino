/*!
   ufire.co for links to documentation, examples, and libraries
   github.com/u-fire for feature requests, bug reports, and  questions
   questions@ufire.co to get in touch with someone

   This example is compatible with Isolated EC Probe Interface
   hardware version 1, firmware version 1.
   
   It demonstrates the minimal amount of code to take a temperature reading.
 */
#include <uFire_EC.h>

uFire_EC ec;

void setup()
{
  Serial.begin(9600);
}

void loop()
{
  ec.measureTemp();
  Serial.print("Temp C|F: ");
  Serial.print(ec.tempC);
  Serial.print("|");
  Serial.println(ec.tempF);
  Serial.println("-----");
  delay(1000);
}
