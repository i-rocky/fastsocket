function compareVersions(a, b) {
  for (var i = 0; i < a.length; i++) {
    if (a[i] < b[i]) { return -1; }
    if (a[i] > b[i]) { return  1; }
  }
  return 0;
}

var logger = new Logger($('#debug-console'), {
  status: true,
  message: true,
  error: true,
  debug: true
});

function logStatus(st) {
  logger.log('status', st);
  $('#status').text(st);
}

function logError(e) {
  logger.log('error', JSON.stringify(e));
}

function logMessage(msg, tag = null) {
  logger.log('message', (tag ? tag  + ': ' : '') + JSON.stringify(msg));
}

function bindTransportCheckboxes(version, encrypted, enabledTransports) {

  if (version >= "3.1.0") {
    var transports = Pusher.Runtime.Transports;
  } else {
    var transports = {
      ws: Pusher.WSTransport,
      flash: Pusher.FlashTransport,
      sockjs: Pusher.SockJSTransport,
      xhr_streaming: Pusher.XHRStreamingTransport,
      xdr_streaming: Pusher.XDRStreamingTransport,
      xhr_polling: Pusher.XHRPollingTransport,
      xdr_polling: Pusher.XDRPollingTransport
    };
  }


  function getCheckboxCallback(checkbox, transport) {
    var isSupportedDefault = transport.isSupported;
    var isSupportedDisabled = function() { return false; };

    return function() {
      if (checkbox.is(":checked")) {
        transport.isSupported = isSupportedDefault;
      } else {
        transport.isSupported = isSupportedDisabled;
      }
    };
  }
  for (var transportName in transports) {
    var transport = transports[transportName];
    if (!transport) {
      continue;
    }

    var env = { encrypted: encrypted };
    var enabled = transport.isSupported(env) && enabledTransports[transportName];
    var checkbox = $("#transport_" + transportName);
    var checkboxCallback = getCheckboxCallback(checkbox, transport);
    checkbox.prop("checked", enabled);
    checkbox.prop("disabled", !transport.isSupported(env));
    checkbox.click(checkboxCallback);
    checkboxCallback(); // update transport status immediately
  }
}

function bindLogCheckboxes(enabledTypes) {
  var types = ["status", "message", "debug", "error"];

  function getCheckboxCallback(checkbox, type) {
    return function() {
      logger.setVisibility(type, checkbox.is(":checked"));
    };
  }
  for (var i = 0; i < types.length; i++) {
    var type = types[i];
    var checkbox = $("#log_" + type);
    var checkboxCallback = getCheckboxCallback(checkbox, type);
    checkbox.prop("checked", enabledTypes[type] !== false);
    checkbox.click(checkboxCallback);
  }
}

function run(env) {
  var pusher;
  var channel;

  $('.ajax').click(function() {
    button = $(this);
    button.addClass('disabled');
    $.post(this.href + '&' + Math.random(), null, function() {
      button.removeClass('disabled');
    });
    return false;
  });

  $('#client').click(function() {
    channel.trigger('client-event', { data: 'hello client' });
  });

  $('#connect').click(function() {
    pusher.connect();
    return false;
  });

  $('#disconnect').click(function() {
    pusher.disconnect();
    return false;
  });

  if (compareVersions(env.version, [1,5,0]) < 0) {
    WebSocket.__swfLocation = "/WebSocketMain.swf";
  }

  Pusher.log = function() {
    if (window.console && window.console.log.apply) {
      window.console.log.apply(window.console, arguments);
    }

    var args = Array.prototype.slice.call(arguments);
    logger.log('debug', args.join(' '));
  };

  // Flash fallback logging
  WEB_SOCKET_DEBUG = true;

  function subscribe(channelName) {
    channel = pusher.subscribe(channelName);
    channel.bind("event", function(data) {
      logMessage(data, channelName);
    });
    channel.bind('alert', function(data) {
      alert(data);
    });
  }

  if (compareVersions(env.version, [1,5,0]) >= 0 && env.encrypted) {
    pusher = new Pusher(env.key, {
      wsHost: '127.0.0.1',
      wsPort: 6002,
      transport: 'ws',
      enabledTransports: ['ws'],
      forceTLS: false,
      encrypted: true,
      auth: {
        params: { "env": env.name }
      },
      cluster: env.name
    });
    subscribe('presence-channel');
    subscribe('public-channel');
    subscribe('private-channel');
    subscribe('private-encrypted-channel');
  } else if (compareVersions(env.version, [1,4,0]) >= 0) {
    pusher = new Pusher(env.key, {
      auth: {
        params: { "env": env.name }
      },
      cluster: env.name
    });
    channel = pusher.subscribe('presence-channel');
    channel.bind("event", function(data) {
      logMessage(data);
    });
    channel.bind('alert', function(data) {
      alert(data);
    });
  } else {
    pusher = new Pusher(env.key, 'presence-channel');
    pusher.bind("event", function(data) {
      logMessage(data);
    });
    pusher.bind('alert', function(data) {
      alert(data);
    });
  }

  if (compareVersions(env.version, [2,0,0]) >= 0) {
    logger.log("debug", "session id: " + pusher.sessionID);
  }

  if (compareVersions(env.version, [1,9,0]) < 0) {
    logStatus('connecting');

    pusher.bind("pusher:connection_established", function() {
      logStatus('connected');
    });
    pusher.bind("connection_established", function() {
      logStatus('connected');
    });

    pusher.bind("pusher:connection_failed", function() {
      logStatus('disconnected');
    });
    pusher.bind("connection_failed", function() {
      logStatus('disconnected');
    });
  } else {
    pusher.connection.bind('state_change', function(state) {
      logStatus(state.current);
    });
    pusher.connection.bind('error', function(e) {
      logError(e);
    });
  }
}
