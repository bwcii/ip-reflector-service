# ip-reflector-service
Returns the public IP address of the endpoint making the call to the service.

Reads the X-Forwarded-For header instead of the source address so that it can work inside of a container in google run.