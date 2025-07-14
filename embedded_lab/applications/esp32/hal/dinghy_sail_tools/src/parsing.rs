use gps_driver::GpsData;
use crate::domain::DataBrooker;
//==================================================================================
pub fn nmea_parsing_bytes<'a>(
    gps_data: &GpsData,
    data_brooker: &'a mut DataBrooker,
) -> &'a DataBrooker {
    data_brooker.position.latitude = gps_data.latitude;
    data_brooker.position.longitude = gps_data.longitude;
    data_brooker.time_stamp.date = gps_data.date;
    data_brooker.time_stamp.time = gps_data.time;
    data_brooker.speed.0 = gps_data.speed_knots;
    data_brooker.cog.0 = gps_data.course;

    data_brooker.altitude = gps_data.altitude;
    if gps_data.sat_count >= Some(6) {
        data_brooker.is_reliable = true
    };

    data_brooker
}
