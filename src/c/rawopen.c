#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>

//int raw_open(const char *pathname, int flags, mode_t mode)
int raw_open(const char *pathname)
{
    return open(pathname, O_RDONLY);
}

int raw_close(int fd)
{
    return close(fd);
}
