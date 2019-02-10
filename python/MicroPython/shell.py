import cmd
from uFire_EC import uFire_EC

# connect board SDA to pin 19, SCL to pin 23, but any pins can be used.
ec = uFire_EC(sda=19, scl=23)


class ECShell(cmd.Cmd):
    prompt = '> '

    def do_config(self, a):
        """prints out all the configuration data\nparameters: none"""
        print("EC Interface Config: " +
              'connected' if ec.connected() else '**disconnected**')
        print("calibration: ")
        print("\toffset: " + str(ec.getCalibrateOffset()))
        print("\tdual point: " + str(ec.usingDualPoint()))
        print("\tlow reference / read: " + str(ec.getCalibrateLowReference()
                                               ) + " / " + str(ec.getCalibrateLowReading()))
        print("\thigh reference / reading: " + str(ec.getCalibrateHighReference()
                                                   ) + " / " + str(ec.getCalibrateHighReading()))
        print("\ttemp. compensation: " + str(ec.usingTemperatureCompensation()))
        print("\t\tconstant: " + str(ec.getTempConstant()))
        print("\tversion: " + (str(ec.getVersion())) +
              "." + (str(ec.getFirmware())))

    def do_reset(self, a):
        """reset all saved values\nparameters: none"""
        ec.reset()

    def do_temp(self, temp_C):
        """measures the temperature\nparameters: none"""
        if temp_C:
            ec.setTemp(float(temp_C))
        else:
            ec.measureTemp()

        print("C/F: " + str(ec.tempC) + " / " + str(ec.tempF))

    def do_raw(self, a):
        """starts an EC measurement\nparameters: none"""
        ec.measureEC()
        print("raw: " + str(ec.raw))

    def do_ec(self, a):
        """starts an EC measurement\nparameters: none"""
        ec.measureEC(False)
        print("mS: " + str(ec.mS))

    def do_tc(self, arg):
        """returns or sets temperature compensation information\nparameters\n\tbool to use compensation\n\ttemperature constant to use (255 for actual)"""
        a = arg.split()

        if len(a) >= 1:
            ec.useTemperatureCompensation(int(a[0]))

        if len(a) >= 2:
            ec.setTempConstant(float(a[1]))

        print("\ttemp. compensation: " + str(ec.usingTemperatureCompensation()))
        print("\t\tconstant: " + str(ec.getTempConstant()))

    def do_low(self, low_reference_mS):
        """returns or sets the low reference/reading values\nparameters\n\tlow reference solution in mS"""
        if low_reference_mS:
            ec.calibrateProbeLow(float(low_reference_mS), ec.tempCoefEC)
        print("\tlow reference / read: " + str(ec.getCalibrateLowReference()
                                               ) + " / " + str(ec.getCalibrateLowReading()))

    def do_high(self, high_reference_mS):
        """returns or sets the high referencen/reading values\nparameters\n\thigh reference solution in mS"""
        if high_reference_mS:
            ec.calibrateProbeHigh(float(high_reference_mS), ec.tempCoefEC)
        print("\thigh reference / reading: " + str(ec.getCalibrateHighReference()
                                                   ) + " / " + str(ec.getCalibrateHighReading()))

    def do_sal(self, a):
        """measures salinity\nparameters: none"""
        ec.measureSalinity(False)
        print("salinity PSU; " + str(ec.salinityPSU))

    def do_dp(self, useDP):
        """returns or sets dual point use\nparameters\n\tbool 0/1"""
        if useDP:
            ec.useDualPoint(int(useDP))
        print("\tdual point: " + str(ec.usingDualPoint()))

    def do_cal(self, solution_mS):
        """calibrates the device\nparameters:\n\tcalibration solution in mS"""
        if solution_mS:
            ec.calibrateProbe(float(solution_mS), ec.tempCoefEC)

        print("offset: " + str(ec.getCalibrateOffset()))

    def do_data(self, a):
        """prints all the data registers"""
        print("raw: " + str(ec.raw))
        print("S: " + str(ec.S))
        print("mS: " + str(ec.mS))
        print("uS: " + str(ec.uS))
        print("TDS 500 | 640 | 700: " + str(ec.PPM_500) +
              " | " + str(ec.PPM_640) + " | " + str(ec.PPM_700))
        print("salinity PSU: " + str(ec.salinityPSU))
        print("C/F: " + str(ec.tempC) + " / " + str(ec.tempF))

    def do_version(self, a):
        """prints the version register"""
        print("\tversion: " + (str(ec.getVersion())) +
              "." + (str(ec.getFirmware())))

    def do_i2c(self, i2cAddress):
        """changes the I2C address"""
        ec.setI2CAddress(i2cAddress)

    def do_read(self, address):
        print(ec.readEEPROM(address))

    def do_write(self, arg):
        a = arg.split()
        ec.writeEEPROM(a[0], a[1])

    def do_EOF(self, line):
        return True


ECShell().cmdloop()
